pub trait UiElement {

    fn render(&self, renderer: &mut Renderer);

}