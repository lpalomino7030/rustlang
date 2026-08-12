use std::sync::Arc;

use crate::renderer::Renderer;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct MinniUi {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl MinniUi {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for MinniUi {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Rust UI"))
                .expect("Failed to create window"),
        );

        let renderer = pollster::block_on(Renderer::new(window.clone()));

        self.renderer = Some(renderer);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_ref() {
                    renderer.render();
                }
            }

            _ => {}
        }
    }
}
