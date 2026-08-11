mod renderer;
mod ui;

use std::sync::Arc;

use crate::renderer::Renderer;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_width = 1200;
        let window_height = 700;

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Mini Engine v0.1")
                        .with_inner_size(winit::dpi::PhysicalSize::new(
                            window_width,
                            window_height,
                        )),
                )
                .expect("Failed to create window"),
        );

        let renderer = pollster::block_on(Renderer::new(&window));

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
                println!("Closing engine...");
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                println!("Window resized: {} x {}", size.width, size.height);
            }

            _ => {}
        }
    }
}

fn main() {
    println!("Starting Mini Engine v0.1...");

    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let mut app = App::new();

    event_loop.run_app(&mut app).expect("Application failed");
}
