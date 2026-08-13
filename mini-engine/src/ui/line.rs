use super::{Bounds, UiElement};

pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub color: [f32; 4],
}

impl UiElement for Line {
    fn bounds(&self) -> Bounds {
        let x = self.start[0].min(self.end[0]);
        let y = self.start[1].min(self.end[1]);

        let width = (self.end[0] - self.start[0]).abs();

        let height = (self.end[1] - self.start[1]).abs();

        Bounds {
            x,
            y,
            width,
            height,
        }
    }
}
