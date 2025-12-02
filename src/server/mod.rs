use bidivec::BidiVec;
use std::collections::HashMap;

pub mod chunk;
pub use chunk::Chunk;

pub mod object;
pub use object::Object;

pub struct Server {
    pub tile_size: u32,
    pub chunks: BidiVec<Chunk>,
    pub objects: HashMap<u32, Object>,
    pub next_object_id: u32,
}

impl Server {
    pub const CHUNK_SIZE: u32 = 16;
    pub const CHUNK_DISTANCE: u32 = 2;
    pub const OBJECT_DISTANCE: f32 = 64.0;

    pub fn new(map: &tiled::Map) -> Self {
        let chunks = BidiVec::with_size_func_xy(
            map.width.div_ceil(Self::CHUNK_SIZE) as _,
            map.height.div_ceil(Self::CHUNK_SIZE) as _,
            |x, y| Chunk::new(&map, (x as _, y as _)),
        );

        Self {
            tile_size: map.tile_width,
            chunks,
            objects: HashMap::new(),
            next_object_id: 1,
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
    pub fn update(&self, buffer: &mut Vec<u8>, position: (f32, f32), self_object: Option<u32>) {
        let self_object = self_object.map(|id| (id, &self.objects[&id]));

        let chunk_coords = |(x, y): (f32, f32)| {
            (
                (x / self.tile_size as f32 / Self::CHUNK_SIZE as f32).floor() as i32,
                (y / self.tile_size as f32 / Self::CHUNK_SIZE as f32).floor() as i32,
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
            let max_dst = |(x, y): (f32, f32)| (object.x - x).abs().max((object.y - y).abs());
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
            }
        }
        buffer.push(0)
    }
}
