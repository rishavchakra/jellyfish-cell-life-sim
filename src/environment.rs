pub struct EnvCell {
    pheromone_level: f32,
}

impl EnvCell {
    pub fn buf_init_desc(width: usize, height: usize) -> wgpu::BufferDescriptor<'static> {
        wgpu::BufferDescriptor {
            label: Some("Env Cell Buffer"),
            size: (width * height * std::mem::size_of::<EnvCell>()) as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
        }
    }

    pub fn compute_shader_desc() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::ShaderModuleDescriptor {
            label: Some("Env Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_compute_env.wgsl").into()),
        }
    }

    pub fn bind_layout_desc() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
            label: Some("Env Compute Bind Group Layout"),
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
                // Env Texture: the compute shader draws to this
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
            ],
        }
    }
}
