use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::agents::{Agent, NUM_AGENTS};
use crate::render_plane::{Vertex, PLANE_VERTICES};

pub struct State<'a> {
    gpu_surface: wgpu::Surface<'a>,
    gpu_device: wgpu::Device,
    gpu_queue: wgpu::Queue,
    gpu_config: wgpu::SurfaceConfiguration,

    bindgroup_plane_env: wgpu::BindGroup,
    bindgroup_plane_agents: wgpu::BindGroup,

    bindgroup_compute_agents: [wgpu::BindGroup; 2],

    pipeline_plane_env: wgpu::RenderPipeline,
    pipeline_plane_agents: wgpu::RenderPipeline,

    pipeline_compute_agents: wgpu::ComputePipeline,

    buf_plane_env: wgpu::Buffer,
    buf_plane_agents: wgpu::Buffer,

    _buf_agent_forward: wgpu::Buffer,
    _buf_agent_reverse: wgpu::Buffer,

    _texture_env: wgpu::Texture,
    _texture_agents: wgpu::Texture,

    window_handle: &'a Window,
    pub window_size: PhysicalSize<u32>,

    frame_num: u64,
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
                    required_features: wgpu::Features::CLEAR_TEXTURE,
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
            .find(|f| f.is_srgb())
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

        let buf_env_vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Plane Vertices Buffer"),
            contents: bytemuck::cast_slice(PLANE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let buf_agent_vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Agent Vertices Buffer"),
            contents: bytemuck::cast_slice(PLANE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let texture_agents = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Agent Texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let texture_agents_view =
            texture_agents.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_agents_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Agent Texture Sampler"),
            address_mode_u: wgpu::AddressMode::MirrorRepeat,
            address_mode_v: wgpu::AddressMode::MirrorRepeat,
            address_mode_w: wgpu::AddressMode::MirrorRepeat,
            mag_filter: wgpu::FilterMode::Linear, // Change to Nearest to retain the pixel look
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_env = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Env Texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let texture_env_view = texture_env.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_env_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Env Texture Sampler"),
            address_mode_u: wgpu::AddressMode::MirrorRepeat,
            address_mode_v: wgpu::AddressMode::MirrorRepeat,
            address_mode_w: wgpu::AddressMode::MirrorRepeat,
            mag_filter: wgpu::FilterMode::Linear, // Change to Nearest to retain the pixel look
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Modules common to both planes
        let plane_bindgroup_layout = device.create_bind_group_layout(&Vertex::bind_layout_desc());
        let plane_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Plane Pipeline Layout"),
                bind_group_layouts: &[&plane_bindgroup_layout],
                push_constant_ranges: &[],
            });

        // Plane-specific modules
        let plane_env_shader = device.create_shader_module(Vertex::shader_env_desc());
        let plane_env_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Env Plane Bindgroup"),
            layout: &plane_bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_env_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_env_sampler),
                },
            ],
        });
        let plane_env_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Env Plane Render Pipeline"),
            layout: Some(&plane_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &plane_env_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::buf_desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &plane_env_shader,
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

        let plane_agent_shader = device.create_shader_module(Vertex::shader_agent_desc());
        let plane_agent_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Agent Plane Bindgroup"),
            layout: &plane_bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_agents_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_agents_sampler),
                },
            ],
        });
        let plane_agent_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Agent Plane Render Pipeline"),
            layout: Some(&plane_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &plane_agent_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::buf_desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &plane_agent_shader,
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

        let buf_agent_forward = device.create_buffer_init(&Agent::buf_init_desc());
        let buf_agent_reverse = device.create_buffer_init(&Agent::buf_init_desc());

        let compute_agent_shader = device.create_shader_module(Agent::compute_shader_desc());
        let compute_agent_bindgroup_layout =
            device.create_bind_group_layout(&Agent::bind_layout_desc());
        let compute_agent_bindgroups = [
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Agent Compute Bindgroup Forward"),
                layout: &compute_agent_bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buf_agent_forward.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: buf_agent_reverse.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&texture_agents_view),
                    },
                ],
            }),
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Agent Compute Bindgroup Reverse"),
                layout: &compute_agent_bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buf_agent_reverse.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: buf_agent_forward.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&texture_agents_view),
                    },
                ],
            }),
        ];
        let compute_agent_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Agent Compute Pipeline Layout"),
                bind_group_layouts: &[&compute_agent_bindgroup_layout],
                push_constant_ranges: &[],
            });
        let compute_agent_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Agent Compute Pipeline"),
                layout: Some(&compute_agent_pipeline_layout),
                module: &compute_agent_shader,
                entry_point: "compute_main",
            });

        Some(Self {
            gpu_surface: surface,
            gpu_device: device,
            gpu_queue: queue,
            gpu_config: config,

            bindgroup_plane_env: plane_env_bindgroup,
            bindgroup_plane_agents: plane_agent_bindgroup,

            bindgroup_compute_agents: compute_agent_bindgroups,

            pipeline_plane_env: plane_env_pipeline,
            pipeline_plane_agents: plane_agent_pipeline,

            pipeline_compute_agents: compute_agent_pipeline,

            buf_plane_env: buf_env_vertices,
            buf_plane_agents: buf_agent_vertices,

            _texture_env: texture_env,
            _texture_agents: texture_agents,

            _buf_agent_forward: buf_agent_forward,
            _buf_agent_reverse: buf_agent_reverse,

            window_handle: window,
            window_size: size,

            frame_num: 0,
        })
    }

    pub fn window(&self) -> &Window {
        self.window_handle
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

            render_pass.set_pipeline(&self.pipeline_plane_env);
            render_pass.set_bind_group(0, &self.bindgroup_plane_env, &[]);
            render_pass.set_vertex_buffer(0, self.buf_plane_env.slice(..));
            render_pass.draw(0..(PLANE_VERTICES.len() as u32), 0..1);

            render_pass.set_pipeline(&self.pipeline_plane_agents);
            render_pass.set_bind_group(0, &self.bindgroup_plane_agents, &[]);
            render_pass.set_vertex_buffer(0, self.buf_plane_agents.slice(..));
            render_pass.draw(0..(PLANE_VERTICES.len() as u32), 0..1);
        }

        {
            encoder.clear_texture(&self._texture_agents, &wgpu::ImageSubresourceRange {
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: None,
                base_array_layer: 0,
                array_layer_count: None,
            });
        }

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Agent Compute Pass"),
                timestamp_writes: None,
            });

            let xdim = NUM_AGENTS as u32;
            let xgroups = xdim / 8;

            compute_pass.set_pipeline(&self.pipeline_compute_agents);
            compute_pass.set_bind_group(
                0,
                &self.bindgroup_compute_agents[(self.frame_num % 2) as usize],
                &[],
            );
            compute_pass.dispatch_workgroups(xgroups, 1, 1);
        }

        self.gpu_queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.frame_num += 1;

        Ok(())
    }
}
