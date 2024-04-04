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
    let prev_cell = env_src[cell_ind];


    let prev_pheromone = prev_cell.pheromone_level;
    var new_pheromone = prev_pheromone;

    var neighborhood_total = 0.0;
    var neighborhood_cells = 0;
    for (var i: i32 = -2; i <= 2; i++) {
        for (var j: i32 = -2; j <= 2; j++) {
            let check_x = u32( i32( cell_x ) + i );
            let check_y = u32( i32( cell_y ) + j );
            if (check_x < 0
            || check_x >= uniforms.dimensions.x
            || check_y < 0
            || check_y >= uniforms.dimensions.y) {
                continue;
            }

            neighborhood_total += env_src[check_x * uniforms.dimensions.x + check_y].pheromone_level;
            neighborhood_cells += 1;
        }
    }
    let neighborhood_blend = neighborhood_total / f32(neighborhood_cells);
    new_pheromone = (0.2 * neighborhood_blend) + (0.8 * new_pheromone);
    new_pheromone = max(0.0, new_pheromone - 0.002);

    var new_cell: EnvCell;
    new_cell.pheromone_level = new_pheromone;

    env_dest[cell_ind] = new_cell;
    textureStore(env_texture,
        vec2<u32>(cell_x, cell_y),
        vec4<f32>(new_pheromone, new_pheromone, new_pheromone, 1.0)
    );
}
