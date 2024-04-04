#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

pub const PLANE_VERTICES: &[Vertex] = &[
    // Triangle 0
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1f32, 1f32],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0f32, 1f32],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0f32, 0f32],
    },
    // Triangle 1
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0f32, 0f32],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1f32, 0f32],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1f32, 1f32],
    },
];

impl Vertex {
    pub fn buf_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                shader_location: 0,
                offset: 0,
                format: wgpu::VertexFormat::Float32x3,
            },
                wgpu::VertexAttribute {
                    shader_location: 1,
                    offset: std::mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ],
        }
    }

    pub fn shader_env_desc() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::ShaderModuleDescriptor {
            label: Some("Plane Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_plane_env.wgsl").into()),
        }
    }

    pub fn shader_agent_desc() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::ShaderModuleDescriptor {
            label: Some("Agent Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_plane_agent.wgsl").into()),
        }
    }

    pub fn bind_layout_desc() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
            label: Some("Plane Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        }
    }
}
