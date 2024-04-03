#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Agent {
    position: [f32; 2],
    angle: f32,
}

const NUM_AGENTS: usize = 100;

lazy_static! {
    pub static ref AGENTS_INIT: [Agent; NUM_AGENTS] = {
        let mut agents_vec = Vec::with_capacity(NUM_AGENTS);

        for _ in 0..NUM_AGENTS {
            agents_vec.push(Agent {
                position: [rand::random(), rand::random()],
                angle: rand::random(),
            })
        }

        agents_vec.try_into().unwrap()
    };
}

impl Agent {
    pub fn buf_layout_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Agent>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                shader_location: 0,
                offset: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }

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

    // pub fn shader_desc() -> wgpu::ShaderModuleDescriptor<'static> {
    //     wgpu::ShaderModuleDescriptor {
    //         label: Some("Agent Shader"),
    //         source: wgpu::ShaderSource::Wgsl(include_str!("shader_agent.wgsl").into()),
    //     }
    // }

    pub fn pipeline_layout_desc() -> wgpu::PipelineLayoutDescriptor<'static> {
        wgpu::PipelineLayoutDescriptor {
            label: Some("Agent Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        }
    }
}
