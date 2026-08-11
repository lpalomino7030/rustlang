use crate::ui::rect::Rect;

pub struct Renderer {
}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_rect(
        &self,
        rect: &Rect,
    ) {
        println!(
            "Rendering Rect: position=({}, {}), size={}x{}",
            rect.x,
            rect.y,
            rect.width,
            rect.height
        );
    }
}