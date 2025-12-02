#[derive(Debug)]
pub struct Object {
    pub x: f32,
    pub y: f32,
    pub client: Option<crate::message::Client>,
}
