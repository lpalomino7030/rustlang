pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub trait UiElement {
    fn bounds(&self) -> Bounds;
}
