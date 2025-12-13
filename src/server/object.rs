#[derive(Debug)]
pub struct Object {
    pub x: u32,
    pub y: u32,
    pub texture: String,
    pub client: Option<crate::message::Client>,
}

impl Object {
    pub fn single_update(
        op: u8,
        id: u32,
        callback: impl FnOnce(&mut Vec<u8>),
    ) -> axum::extract::ws::Message {
        let mut msg = vec![b'u', 0, op];
        msg.extend_from_slice(&id.to_be_bytes());
        callback(&mut msg);
        msg.push(0);
        axum::extract::ws::Message::binary(msg)
    }

    pub fn visible(&self, (x, y): (u32, u32)) -> bool {
        self.x.abs_diff(x).max(self.y.abs_diff(y)) <= super::Server::OBJECT_DISTANCE
    }

    pub fn send(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.x.to_be_bytes());
        buffer.extend_from_slice(&self.y.to_be_bytes());
        buffer.extend_from_slice(&self.texture.as_bytes());
        buffer.push(0);
    }
}
