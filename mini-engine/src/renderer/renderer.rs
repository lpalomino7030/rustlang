use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::math::coordinates::screen_to_ndc;
use crate::ui::Line;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

pub struct Renderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    vertex_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        println!("Creating GPU renderer...");

        // --------------------------------------------------
        // 1. INSTANCE
        // --------------------------------------------------

        let instance = wgpu::Instance::default();

        // --------------------------------------------------
        // 2. SURFACE
        // --------------------------------------------------

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        // --------------------------------------------------
        // 3. ADAPTER
        // --------------------------------------------------

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

        // --------------------------------------------------
        // 4. DEVICE + QUEUE
        // --------------------------------------------------

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

        // --------------------------------------------------
        // 5. SURFACE CONFIGURATION
        // --------------------------------------------------

        let size = window.inner_size();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Surface is not supported by adapter");

        surface.configure(&device, &config);

        println!("Surface configured: {}x{}", config.width, config.height);

        // --------------------------------------------------
        // 6. SHADER
        // --------------------------------------------------

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("UI Shader"),

            source: wgpu::ShaderSource::Wgsl(include_str!("../assets/shaders/ui.wgsl").into()),
        });

        // --------------------------------------------------
        // 7. VERTICES
        // --------------------------------------------------

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Line Vertex Buffer"),

            size: (std::mem::size_of::<Vertex>() * 2) as wgpu::BufferAddress,

            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,

            mapped_at_creation: false,
        });
        // --------------------------------------------------
        // 8. VERTEX BUFFER
        // --------------------------------------------------

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,

            step_mode: wgpu::VertexStepMode::Vertex,

            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,

                    offset: 0,

                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,

                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,

                    shader_location: 1,
                },
            ],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Pipeline"),

            layout: None,

            vertex: wgpu::VertexState {
                module: &shader,

                entry_point: Some("vs_main"),

                compilation_options: wgpu::PipelineCompilationOptions::default(),

                buffers: &[Some(vertex_layout)],
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,

                entry_point: Some("fs_main"),

                compilation_options: wgpu::PipelineCompilationOptions::default(),

                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,

                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),

                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,

                strip_index_format: None,

                front_face: wgpu::FrontFace::Ccw,

                cull_mode: None,

                polygon_mode: wgpu::PolygonMode::Fill,

                unclipped_depth: false,

                conservative: false,
            },

            depth_stencil: None,

            multisample: wgpu::MultisampleState::default(),

            multiview_mask: None,

            cache: None,
        });

        println!("GPU device created successfully.");

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            vertex_buffer,
            pipeline,
        }
    }

    pub fn render(&self) {
        println!("Rendering frame...");

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => {
                println!("Surface texture: Success");

                output
            }

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                println!("Surface texture: Suboptimal");

                output
            }

            wgpu::CurrentSurfaceTexture::Timeout => {
                eprintln!("Surface texture: Timeout");

                return;
            }

            wgpu::CurrentSurfaceTexture::Occluded => {
                eprintln!("Surface texture: Occluded");

                return;
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                eprintln!("Surface texture: Outdated");

                return;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                eprintln!("Surface texture: Lost");

                return;
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                eprintln!("Surface texture: Validation");

                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Line Pass"),

                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,

                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.08,
                            b: 0.12,
                            a: 1.0,
                        }),

                        store: wgpu::StoreOp::Store,
                    },
                })],

                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            render_pass.draw(0..2, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        self.queue.present(output);
    }

    pub fn render_line(&self, line: &Line) {
        let width = self.config.width as f32;

        let height = self.config.height as f32;
        let start = screen_to_ndc(line.start[0], line.start[1], width, height);

        let end = screen_to_ndc(line.end[0], line.end[1], width, height);

        let vertices = [
            Vertex {
                position: start,
                color: line.color,
            },
            Vertex {
                position: end,
                color: line.color,
            },
        ];

        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
    }
}
