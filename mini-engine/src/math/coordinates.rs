pub fn screen_to_ndc(x: f32, y: f32, width: f32, height: f32) -> [f32; 2] {
    let ndc_x = (x / width) * 2.0 - 1.0;

    let ndc_y = 1.0 - (y / height) * 2.0;

    [ndc_x, ndc_y]
}
