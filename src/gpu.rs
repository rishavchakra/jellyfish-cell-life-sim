use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

pub struct State<'a> {
    gpu_surface: wgpu::Surface<'a>,
    gpu_device: wgpu::Device,
    gpu_queue: wgpu::Queue,
    gpu_config: wgpu::SurfaceConfiguration,

    window_handle: &'a Window,
    pub window_size: PhysicalSize<u32>,
}

impl<'a> State<'a> {
    pub async fn new(window: &'a Window) -> Option<Self> {
        let size = window.inner_size();
        if size.height == 0 || size.width == 0 {
            return None;
        }

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window).ok()?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .ok()?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };

        surface.configure(&device, &config);

        Some(Self {
            gpu_surface: surface,
            gpu_device: device,
            gpu_queue: queue,
            gpu_config: config,
            window_handle: window,
            window_size: size,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window_handle
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.window_size = new_size;
        self.gpu_config.width = new_size.width;
        self.gpu_config.height = new_size.height;
        self.gpu_surface
            .configure(&self.gpu_device, &self.gpu_config);
    }

    pub fn input_is_handled(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.gpu_surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .gpu_device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.gpu_queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
