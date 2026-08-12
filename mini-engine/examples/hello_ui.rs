use std::sync::Arc;

use mini_engine::renderer::Renderer;
use mini_engine::ui::Line;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct ExampleApp {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl ExampleApp {
    fn new() -> Self {
        Self {
            window: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for ExampleApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Hello UI"))
                .expect("Failed to create window"),
        );

        let renderer = pollster::block_on(Renderer::new(window.clone()));

        let line = Line {
            start: [100.0, 100.0],
            end: [500.0, 300.0],
            color: [0.0, 1.0, 0.0, 1.0],
        };

        renderer.render_line(&line);

        window.request_redraw();

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

fn main() {
    println!("================================");
    println!("ESTOY EJECUTANDO HELLO_UI");
    println!("================================");

    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let mut app = ExampleApp::new();

    event_loop.run_app(&mut app).expect("Application failed");
}
