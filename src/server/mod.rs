use bidivec::BidiVec;
use std::collections::HashMap;

pub mod chunk;
pub use chunk::Chunk;

pub mod object;
pub use object::Object;

fn image_to_texture(image: &tiled::Image) -> String {
    let path = image.source.as_path();
    path.iter()
        .skip_while(|c| *c != "assets")
        .skip(1)
        .collect::<std::path::PathBuf>()
        .to_string_lossy()
        .into_owned()
}

fn tileset(tileset: &tiled::Tileset) -> String {
    image_to_texture(tileset.image.as_ref().unwrap())
}

pub struct Server {
    pub tileset: String,
    pub tile_size: u32,
    pub offsets: Vec<u32>,
    pub chunks: BidiVec<Chunk>,
    pub objects: HashMap<u32, Object>,
    pub next_object_id: u32,
}

impl Server {
    pub const CHUNK_SIZE: u32 = 16;
    pub const CHUNK_DISTANCE: u32 = 2;
    pub const OBJECT_DISTANCE: u32 = 48;

    pub fn new(map: &tiled::Map) -> Self {
        let chunks = BidiVec::with_size_func_xy(
            map.width.div_ceil(Self::CHUNK_SIZE) as _,
            map.height.div_ceil(Self::CHUNK_SIZE) as _,
            |x, y| Chunk::new(&map, (x as _, y as _)),
        );

        let mut objects = HashMap::new();
        let mut next_object_id = 1;
        for layer in map.layers() {
            let Some(layer) = layer.as_object_layer() else {
                continue;
            };

            for object in layer.objects() {
                let tile = object.get_tile().unwrap().get_tile().unwrap();
                let tile = tile.image.as_ref().unwrap();
                objects.insert(
                    object.id(),
                    Object {
                        x: (object.x as i32 + tile.width / 2) as u32 / map.tile_width,
                        y: object.y as u32 / map.tile_height,
                        texture: image_to_texture(tile),
                        client: None,
                    },
                );
                next_object_id = next_object_id.max(object.id() + 1);
            }
        }

        Self {
            tileset: tileset(&map.tilesets()[0]),
            tile_size: map.tile_width,
            offsets: map
                .layers()
                .filter(|layer| layer.as_tile_layer().is_some())
                .map(|layer| layer.offset_x as _)
                .collect(),
            chunks,
            objects,
            next_object_id,
        }
    }

    pub fn spawn(&mut self, object: Object) -> u32 {
        let id = self.next_object_id;
        self.next_object_id += 1;

        let msg = Object::single_update(b'+', id, |buffer| object.send(buffer));
        for receiver in self.objects.values() {
            let Some(client) = &receiver.client else {
                continue;
            };

            let visible = receiver.visible((object.x, object.y));
            if visible {
                crate::log_err!(client.send(msg.clone()));
            }
        }

        self.objects.insert(id, object);
        id
    }

    pub fn despawn(&mut self, id: u32) {
        let object = self.objects.remove(&id).unwrap();

        let msg = Object::single_update(b'-', id, |_| ());
        for receiver in self.objects.values() {
            let Some(client) = &receiver.client else {
                continue;
            };

            let visible = receiver.visible((object.x, object.y));
            if visible {
                crate::log_err!(client.send(msg.clone()));
            }
        }
    }

    pub fn move_object(&mut self, id: u32, new_pos: (u32, u32)) {
        let object = self.objects.get_mut(&id).unwrap();
        let old_pos = (object.x, object.y);
        (object.x, object.y) = new_pos;

        let msg_add = Object::single_update(b'+', id, |buffer| object.send(buffer));
        let msg_del = Object::single_update(b'-', id, |_| ());
        let msg_upd = Object::single_update(b'u', id, |buffer| {
            buffer.extend_from_slice(&object.x.to_be_bytes());
            buffer.extend_from_slice(&object.y.to_be_bytes());
        });

        for (rec_id, receiver) in &self.objects {
            if *rec_id == id {
                continue;
            }
            let Some(client) = &receiver.client else {
                continue;
            };

            let last_visible = receiver.visible(old_pos);
            let visible = receiver.visible(new_pos);
            match (last_visible, visible) {
                (false, true) => crate::log_err!(client.send(msg_add.clone())),
                (true, false) => crate::log_err!(client.send(msg_del.clone())),
                (true, true) => crate::log_err!(client.send(msg_upd.clone())),
                (false, false) => None,
            };
        }
    }

    /// Generate update packet into buffer, as if the client moved from self_object's position to position.
    /// If self_object is None, it is a join packet
    pub fn update(&self, buffer: &mut Vec<u8>, position: (u32, u32), self_object: Option<u32>) {
        let self_object = self_object.map(|id| (id, &self.objects[&id]));

        // **** Chunks
        let chunk_coords =
            |(x, y): (u32, u32)| ((x / Self::CHUNK_SIZE) as i32, (y / Self::CHUNK_SIZE) as i32);
        let center = chunk_coords(position);
        let cdst_range = |c: i32| c - Self::CHUNK_DISTANCE as i32..=c + Self::CHUNK_DISTANCE as i32;
        for y in cdst_range(center.1) {
            for x in cdst_range(center.0) {
                if x < 0
                    || y < 0
                    || x as u32 >= self.chunks.width() as u32
                    || y as u32 >= self.chunks.height() as u32
                {
                    continue;
                }

                // Skip chunks that were visible from the last position
                if let Some((_, object)) = self_object {
                    let last_center = chunk_coords((object.x, object.y));
                    if cdst_range(last_center.0).contains(&x)
                        && cdst_range(last_center.1).contains(&y)
                    {
                        continue;
                    }
                }

                // Add the new chunk
                self.chunks[(x as _, y as _)].send(buffer, (x as _, y as _));
            }
        }
        buffer.push(0);

        // **** Objects
        for (id, object) in &self.objects {
            // Skip self
            if let Some((self_id, _)) = self_object {
                if *id == self_id {
                    continue;
                }
            }

            // Get visibility status
            let visible = object.visible(position);
            let last_visible = if let Some((_, self_object)) = self_object {
                object.visible((self_object.x, self_object.y))
            } else {
                false
            };

            if visible == last_visible {
                continue;
            }

            buffer.push(if visible { b'+' } else { b'-' });
            buffer.extend_from_slice(&id.to_be_bytes());
            if visible {
                object.send(buffer);
            }
        }
        buffer.push(0)
    }

    pub fn interact(&mut self, id: u32, player_id: u32) {
        let [Some(obj), Some(plr)] = self.objects.get_disjoint_mut([&id, &player_id]) else {
            return;
        };
        if plr.x != obj.x || plr.y != obj.y {
            return;
        }
        if obj.texture == "objects/pine.png" {
            self.despawn(id);
        }
    }
}
