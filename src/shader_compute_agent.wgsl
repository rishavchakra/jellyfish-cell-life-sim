struct Agent {
    position: vec2<f32>,
    angle: f32,
};

struct EnvCell {
    pheromone_level: f32,
}

struct Uniforms {
    dimensions: vec2<u32>,
}

struct ComputeInput {
@builtin(global_invocation_id) global_id: vec3<u32>,
};

fn hash_2d(in: vec2<f32>) -> f32 {
    return fract(sin(dot(in, vec2(12.9898, 78.233))) * 43758.5453);
}

@group(0) @binding(0) var<storage, read> agent_src: array<Agent>;
@group(0) @binding(1) var<storage, read_write> agent_dest: array<Agent>;
@group(0) @binding(2) var agent_texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(4) var<storage, read_write> env_dest: array<EnvCell>;

@group(1) @binding(0) var<uniform> uniforms: Uniforms;

@compute
@workgroup_size(8, 1, 1)
fn compute_main(
    in: ComputeInput,
) {
    let agent_id = in.global_id.x;
    let agent_x = agent_src[agent_id].position.x;
    let agent_y = agent_src[agent_id].position.y;
    let agent_angle = agent_src[agent_id].angle;

    let speed = 1.0;

    var new_agent: Agent;
    new_agent.position.x = speed * cos(agent_angle) + agent_x;
    new_agent.position.y = speed * sin(agent_angle) + agent_y;
    new_agent.angle = agent_angle;
    
    if (new_agent.position.x < 0.0
        || new_agent.position.y < 0.0
        || u32(new_agent.position.x) > uniforms.dimensions.x
        || u32(new_agent.position.y) > uniforms.dimensions.y) {
        new_agent.position = agent_src[agent_id].position;
        new_agent.angle = hash_2d(new_agent.position + new_agent.angle) * 6.28;
    }
    let agent_index = u32(new_agent.position.x) * uniforms.dimensions.x + u32(new_agent.position.y);

    agent_dest[agent_id] = new_agent;
    env_dest[agent_index].pheromone_level = 1.0;
    textureStore(agent_texture,
        vec2<i32>(i32(new_agent.position.x), i32(new_agent.position.y)),
        vec4<f32>(1.0, 1.0, 1.0, 1.0)
    );
}
