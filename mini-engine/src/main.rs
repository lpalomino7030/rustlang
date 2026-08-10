// use winit::{
//     application::ApplicationHandler,
//     event::WindowEvent,
//     event_loop::{
//         ActiveEventLoop,
//         EventLoop,
//     },
//     window::{
//         Window,
//         WindowId,
//     },
// };

// struct App {
//     window: Option<Window>,
// }

// impl App {
//     fn new() -> Self {
//         Self {
//             window: None,
//         }
//     }
// }

// impl ApplicationHandler for App {
//     fn resumed(
//         &mut self,
//         event_loop: &ActiveEventLoop,
//     ) {
//         if self.window.is_some() {
//             return;
//         }

//         let window = event_loop
//             .create_window(
//                 Window::default_attributes()
//                     .with_title(
//                         "Mini Engine v0.1"
//                     )
//                     .with_inner_size(
//                         winit::dpi::PhysicalSize::new(
//                             800,
//                             600,
//                         )
//                     ),
//             )
//             .expect(
//                 "Failed to create window"
//             );

//         self.window = Some(window);
//     }

//     fn window_event(
//         &mut self,
//         event_loop: &ActiveEventLoop,
//         window_id: WindowId,
//         event: WindowEvent,
//     ) {
//         let Some(window) =
//             self.window.as_ref()
//         else {
//             return;
//         };

//         if window.id() != window_id {
//             return;
//         }

//         match event {
//             WindowEvent::CloseRequested => {
//                 println!(
//                     "Closing engine..."
//                 );

//                 event_loop.exit();
//             }

//             _ => {}
//         }
//     }
// }

// fn main() {
//     println!(
//         "Starting Mini Engine v0.1..."
//     );

//     let event_loop =
//         EventLoop::new()
//             .expect(
//                 "Failed to create event loop"
//             );

//     let mut app = App::new();

//     event_loop
//         .run_app(&mut app)
//         .expect(
//             "Application failed"
//         );
// }
fn main() {
    println!("Mini Engine - GPU Probe");

    pollster::block_on(probe_gpu());
}

async fn probe_gpu() {
    let instance = wgpu::Instance::default();

    println!();
    println!("Available adapters:");
    println!("--------------------");

    let adapters = instance
        .enumerate_adapters(wgpu::Backends::all())
        .await;

    if adapters.is_empty() {
        println!("No GPU adapters found.");
        return;
    }

    for adapter in adapters {
        let info = adapter.get_info();

        println!("Name:    {}", info.name);
        println!("Vendor:  {}", info.vendor);
        println!("Device:  {}", info.device);
        println!("Backend: {:?}", info.backend);
        println!("Type:    {:?}", info.device_type);
        println!();
    }
}