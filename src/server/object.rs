#[derive(Debug)]
pub struct Object {
    pub x: u32,
    pub y: u32,
    pub texture: String,
    pub client: Option<crate::message::Client>,
}
