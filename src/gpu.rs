use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::agents::AgentVert;
use crate::render_plane::Vertex;

pub struct State<'a> {
    gpu_surface: wgpu::Surface<'a>,
    gpu_device: wgpu::Device,
    gpu_queue: wgpu::Queue,
    gpu_config: wgpu::SurfaceConfiguration,

    pipeline_plane: wgpu::RenderPipeline,
    pipeline_agents: wgpu::RenderPipeline,

    buf_plane_vertices: wgpu::Buffer,
    buf_agent_vertices: wgpu::Buffer,

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

        let plane_shader = device.create_shader_module(Vertex::shader_desc());
        let plane_pipeline_layout = device.create_pipeline_layout(&Vertex::pipeline_layout_desc());
        let plane_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Plane Render Pipeline"),
            layout: Some(&plane_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &plane_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::buf_desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &plane_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: None,
            multiview: None,
        });

        let agent_shader = device.create_shader_module(AgentVert::shader_desc());
        let agent_pipeline_layout =
            device.create_pipeline_layout(&AgentVert::pipeline_layout_desc());
        let agent_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Agent Render Pipeline"),
            layout: Some(&agent_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &agent_shader,
                entry_point: "vs_main",
                buffers: &[AgentVert::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &agent_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: None,
            multiview: None,
        });

        let buf_plane_vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Plane Vertices Buffer"),
            contents: bytemuck::cast_slice(crate::render_plane::PLANE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let buf_agent_vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Agent Vertices Buffer"),
            contents: bytemuck::cast_slice(crate::agents::AGENT_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Some(Self {
            gpu_surface: surface,
            gpu_device: device,
            gpu_queue: queue,
            gpu_config: config,

            pipeline_plane: plane_pipeline,
            pipeline_agents: agent_pipeline,

            buf_plane_vertices,
            buf_agent_vertices,

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

    pub fn input_is_handled(&mut self, _event: &WindowEvent) -> bool {
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
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Plane Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    // Draw to the screen texture view
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

            render_pass.set_pipeline(&self.pipeline_plane);
            render_pass.set_vertex_buffer(0, self.buf_plane_vertices.slice(..));
            render_pass.draw(0..(3 * 2), 0..1);

            // render_pass.set_pipeline(&self.buf_agent_vertices);
            render_pass.set_pipeline(&self.pipeline_agents);
            render_pass.set_vertex_buffer(0, self.buf_agent_vertices.slice(..));
            render_pass.draw(0..3, 0..1);
        }

        {
            // let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            //     label: Some("Agents Render Pass"),
            //     color_attachments: &[Some(wgpu::RenderP  {
            //         view: &view,
            //         resolve_target: None,
            //         ops: wgpu::Operations {
            //             load: wgpu::LoadOp::Load,
            //             store: wgpu::StoreOp::Store,
            //         },
            //     })],
            //     depth_stencil_attachment: None,
            //     occlusion_query_set: None,
            //     timestamp_writes: None,
            // });
        }

        self.gpu_queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
