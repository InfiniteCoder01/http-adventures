use bidivec::BidiVec;

pub mod chunk;
pub use chunk::Chunk;

pub struct Server {
    pub tile_size: u32,
    pub chunks: BidiVec<Chunk>,
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
        }
    }

    /// Generate update packet into buffer, as if the client moved from last_position to position.
    /// If last_position is None, it is a join packet
    pub fn update(
        &self,
        buffer: &mut Vec<u8>,
        position: (f32, f32),
        last_position: Option<(f32, f32)>,
    ) {
        let chunk_coords = |(x, y): (f32, f32)| {
            (
                (x / self.tile_size as f32 / Self::CHUNK_SIZE as f32).floor() as i32,
                (y / self.tile_size as f32 / Self::CHUNK_SIZE as f32).floor() as i32,
            )
        };

        let center = chunk_coords(position);

        let cdst_range = |c: i32| c - Self::CHUNK_DISTANCE as i32..c + Self::CHUNK_DISTANCE as i32;
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
                if let Some(last_position) = last_position {
                    let last_center = chunk_coords(last_position);
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
    }
}
