use std::sync::Arc;

use mini_engine::renderer::Renderer;
use mini_engine::ui::{
    Line,
    Rect,
    UiElement,
};

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

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Rust UI")
                        .with_inner_size(winit::dpi::PhysicalSize::new(1200, 700)),
                )
                .expect("Failed to create window"),
        );

        let renderer = pollster::block_on(Renderer::new(window.clone()));

        let line = Line {
            start: [50.0, 600.0],
            end: [1100.0, 100.0],
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
                println!("Closing engine...");
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                println!("Window resized: {} x {}", size.width, size.height);
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

fn test_elements() {
    let line = Line {
        start: [100.0, 100.0],
        end: [500.0, 300.0],
        color: [1.0, 0.0, 0.0, 1.0],
    };

    let rect = Rect {
        x: 200.0,
        y: 200.0,
        width: 300.0,
        height: 100.0,
        color: [0.0, 1.0, 0.0, 1.0],
    };

    let elements: Vec<Box<dyn UiElement>> = vec![
        Box::new(line),
        Box::new(rect),
    ];

    for element in &elements {
        let bounds = element.bounds();

        println!(
            "Element bounds: x={}, y={}, width={}, height={}",
            bounds.x,
            bounds.y,
            bounds.width,
            bounds.height,
        );
    }
}

fn main() {
    println!("Starting Mini Engine v0.1...");

    test_elements();

    let event_loop =
        EventLoop::new()
            .expect("Failed to create event loop");

    let mut app = App::new();

    event_loop
        .run_app(&mut app)
        .expect("Application failed");
}