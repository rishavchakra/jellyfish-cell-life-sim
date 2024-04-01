#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AgentVert {
    position: [f32; 2],
}

const AGENT_SIZE: f32 = 0.2;

/// Equilateral triangle centered at the origin
pub const AGENT_VERTICES: &[AgentVert] = &[
    AgentVert {
        position: [0f32, AGENT_SIZE * 0.433],
    },
    AgentVert {
        position: [AGENT_SIZE * -0.5, AGENT_SIZE * -0.433],
    },
    AgentVert {
        position: [AGENT_SIZE * 0.5, AGENT_SIZE * -0.433],
    },
];

impl AgentVert {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<AgentVert>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                shader_location: 0,
                offset: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }

    pub fn shader_desc() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::ShaderModuleDescriptor {
            label: Some("Agent Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_agent.wgsl").into()),
        }
    }

    pub fn pipeline_layout_desc() -> wgpu::PipelineLayoutDescriptor<'static> {
        wgpu::PipelineLayoutDescriptor {
            label: Some("Agent Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AgentInstance {
    position: [f32; 2],
    direction: f32,
}

impl AgentInstance {
    pub fn buf_desc() -> wgpu::VertexBufferLayout<'static> {
        todo!()
    }
}
