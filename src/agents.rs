use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Agent {
    position: [f32; 2],
    angle: f32,
    turn_speed: f32,
}

pub const NUM_AGENTS: usize = 10_000;

lazy_static! {
    pub static ref AGENTS_INIT: [Agent; NUM_AGENTS] = {
        let mut agents_vec = Vec::with_capacity(NUM_AGENTS);
        let mut rng = thread_rng();
        let position_range = Uniform::from(200..=800);
        let angle_range = Uniform::from(0.0..std::f32::consts::TAU);

        for _ in 0..NUM_AGENTS {
            let x = position_range.sample(&mut rng) as f32;
            let y = position_range.sample(&mut rng) as f32;
            // println!("{}, {}", x, y);
            agents_vec.push(Agent {
                position: [x, y],
                angle: angle_range.sample(&mut rng),
                turn_speed: 0f32,
            })
        }

        agents_vec.try_into().unwrap()
    };
}

impl Agent {
    pub fn buf_init_desc() -> wgpu::util::BufferInitDescriptor<'static> {
        wgpu::util::BufferInitDescriptor {
            label: Some("Agent Buffer"),
            contents: bytemuck::cast_slice(&*AGENTS_INIT),
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
        }
    }

    pub fn compute_shader_desc() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::ShaderModuleDescriptor {
            label: Some("Agent Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_compute_agent.wgsl").into()),
        }
    }

    pub fn bind_layout_desc() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
            label: Some("Agent Compute Bind Group Layout"),
            entries: &[
                // Source buffer: the compute shader reads from this
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Destination buffer: the compute shader writes to this (for the next frame's compute)
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Agent Texture: the compute shader writes to this (for display)
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                // Source Env Buffer: the compute shader reads the environment from these
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Destination Env Buffer: the compute shader leaves a trail on this
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        }
    }
}
