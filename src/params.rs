pub struct Params {
    pub agent_compute_params: AgentComputeParams,
    pub agent_render_params: AgentRenderParams,
    pub env_compute_params: EnvComputeParams,
    pub env_render_params: EnvRenderParams,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct AgentComputeParams {
    dimensions: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct AgentRenderParams {
    a: f32
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct EnvComputeParams {
    a: f32
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct EnvRenderParams {
    a: f32
}

impl Params {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            agent_compute_params: AgentComputeParams {
                dimensions: [width, height],
            },
            agent_render_params: AgentRenderParams {
                a: 0.0,
            },
            env_compute_params: EnvComputeParams {
                a: 0.0,
            },
            env_render_params: EnvRenderParams {
                a: 0.0,
            },
        }
    }
}

impl AgentComputeParams {
    pub fn bind_layout_desc() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
            label: Some("Agent Compute Uniform Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ]
        }
    }
}
