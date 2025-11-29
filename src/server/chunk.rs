use super::Server;

pub struct Tile(pub u32);
impl Tile {
    const BLANK: Self = Self(u32::MAX);

    pub fn new(tile: tiled::LayerTile) -> Self {
        Self(tile.id())
    }
}

pub struct Chunk {
    layers: Vec<Vec<Tile>>,
}

impl Chunk {
    pub fn new(map: &tiled::Map, coords: (u32, u32)) -> Self {
        let mut layers = Vec::new();
        for layer in map.layers() {
            let Some(layer) = layer.as_tile_layer() else {
                continue;
            };
            let mut tiles = Vec::with_capacity(Server::CHUNK_SIZE.pow(2) as _);
            let crange = |c| c * Server::CHUNK_SIZE..(c + 1) * Server::CHUNK_SIZE;
            for y in crange(coords.1) {
                for x in crange(coords.0) {
                    let tile = layer.get_tile(x as _, y as _);
                    tiles.push(tile.map_or(Tile::BLANK, Tile::new));
                }
            }
            layers.push(tiles);
        }
        Self { layers }
    }

    pub fn send(&self, buffer: &mut Vec<u8>, coords: (u32, u32)) {
        buffer.push(self.layers.len() as u8);
        buffer.extend_from_slice(&coords.0.to_be_bytes());
        buffer.extend_from_slice(&coords.1.to_be_bytes());
        for layer in &self.layers {
            for tile in layer {
                buffer.extend_from_slice(&tile.0.to_be_bytes());
            }
        }
    }
}
