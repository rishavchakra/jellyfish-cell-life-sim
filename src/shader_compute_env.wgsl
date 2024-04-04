struct EnvCell {
    pheromone_level: f32,
}

struct Uniforms {
    dimensions: vec2<u32>,
}

struct ComputeInput {
    @builtin(global_invocation_id) global_id: vec3<u32>,
}

@group(0) @binding(0) var<storage, read> env_src: array<EnvCell>;
@group(0) @binding(1) var<storage, read_write> env_dest: array<EnvCell>;
@group(0) @binding(2) var env_texture: texture_storage_2d<rgba8unorm, write>;

@group(1) @binding(0) var<uniform> uniforms: Uniforms;

@compute
@workgroup_size(8, 8, 1)
fn compute_main(
    in: ComputeInput,
) {
    let cell_x = in.global_id.x;
    let cell_y = in.global_id.y;

    let cell_ind = cell_x * uniforms.dimensions.x + cell_y;
    let prev_env_val = env_src[cell_ind];
    let prev_pheromone = prev_env_val.pheromone_level;

    textureStore(env_texture,
        vec2<u32>(cell_x, cell_y),
        vec4<f32>(prev_pheromone, prev_pheromone, prev_pheromone, 1.0)
    );
}
