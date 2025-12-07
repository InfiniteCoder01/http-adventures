#[derive(Debug)]
pub struct Object {
    pub x: f32,
    pub y: f32,
    pub texture: String,
    pub client: Option<crate::message::Client>,
}
