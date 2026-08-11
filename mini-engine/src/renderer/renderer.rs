use winit::window::Window;

pub struct Renderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Renderer {
    pub async fn new(_window: &Window) -> Self {
        println!("Creating GPU renderer...");

        let instance = wgpu::Instance::default();

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,

                compatible_surface: Some(&surface),

                force_fallback_adapter: false,

                apply_limit_buckets: false,
            })
            .await
            .expect("Failed to find a suitable GPU adapter");

        let info = adapter.get_info();

        println!("GPU: {}", info.name);
        println!("Backend: {:?}", info.backend);
        println!("Device type: {:?}", info.device_type);

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Mini UI Device"),

                required_features: wgpu::Features::empty(),

                required_limits: wgpu::Limits::default(),

                experimental_features: wgpu::ExperimentalFeatures::disabled(),

                memory_hints: wgpu::MemoryHints::Performance,

                trace: wgpu::Trace::Off,
            })
            .await
            .expect("Failed to create GPU device");

        println!("GPU device created successfully.");

        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }
}
