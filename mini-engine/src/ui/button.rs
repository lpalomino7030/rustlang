pub struct Button {
    text: String,
    width: f32,
    height: f32,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            width: 120.0,
            height: 40.0,
        }
    }
}
