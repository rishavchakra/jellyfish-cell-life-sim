#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

pub const PLANE_VERTICES: &[Vertex] = &[
    // Triangle 0
    Vertex {
        position: [0.5, 0.5, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
    },
    // Triangle 1
    Vertex {
        position: [-0.5, -0.5, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
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
            }],
        }
    }

    pub fn shader_desc() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::ShaderModuleDescriptor {
            label: Some("Plane Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_plane.wgsl").into()),
        }
    }

    pub fn pipeline_layout_desc() -> wgpu::PipelineLayoutDescriptor<'static> {
        wgpu::PipelineLayoutDescriptor {
            label: Some("Plane Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        }
    }
}
