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
        self.objects.insert(id, object);
        id
    }

    pub fn despawn(&mut self, id: u32) {
        self.objects.remove(&id);
    }

    /// Generate update packet into buffer, as if the client moved from self_object's position to position.
    /// If self_object is None, it is a join packet
    pub fn update(&self, buffer: &mut Vec<u8>, position: (u32, u32), self_object: Option<u32>) {
        let self_object = self_object.map(|id| (id, &self.objects[&id]));

        let chunk_coords = |(x, y): (u32, u32)| {
            (
                (x / self.tile_size / Self::CHUNK_SIZE) as i32,
                (y / self.tile_size / Self::CHUNK_SIZE) as i32,
            )
        };

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

        for (id, object) in &self.objects {
            // Skip self
            if let Some((self_id, _)) = self_object {
                if *id == self_id {
                    continue;
                }
            }

            // Get visibility status
            let max_dst = |(x, y): (u32, u32)| object.x.abs_diff(x).max(object.y.abs_diff(y));
            let visible = max_dst(position) <= Self::OBJECT_DISTANCE;
            let last_visible = if let Some((_, self_object)) = self_object {
                max_dst((self_object.x, self_object.y)) <= Self::OBJECT_DISTANCE
            } else {
                false
            };

            if visible == last_visible {
                continue;
            }

            buffer.push(if visible { b'+' } else { b'-' });
            buffer.extend_from_slice(&id.to_be_bytes());
            if visible {
                buffer.extend_from_slice(&object.x.to_be_bytes());
                buffer.extend_from_slice(&object.y.to_be_bytes());
                buffer.extend_from_slice(&object.texture.as_bytes());
                buffer.push(0);
            }
        }
        buffer.push(0)
    }
}
