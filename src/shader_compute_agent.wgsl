struct Agent {
    position: vec2<f32>,
    angle: f32,
};

struct Uniforms {
    dimensions: vec2<u32>,
}

struct ComputeInput {
@builtin(global_invocation_id) global_id: vec3<u32>,
};

fn hash_2d(in: vec2<f32>) -> f32 {
    return fract(sin(dot(in, vec2(12.9898, 78.233))) * 43758.5453);
}

@group(0) @binding(0) var<storage, read> agentSrc: array<Agent>;
@group(0) @binding(1) var<storage, read_write> agentDest: array<Agent>;
@group(0) @binding(2) var agent_texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(4) var env_texture: texture_storage_2d<rgba8unorm, write>;

@group(1) @binding(0) var<uniform> uniforms: Uniforms;

@compute
@workgroup_size(8, 1, 1)
fn compute_main(
    in: ComputeInput,
) {
    let agent_id = in.global_id.x;
    let agent_x = agentSrc[agent_id].position.x;
    let agent_y = agentSrc[agent_id].position.y;
    let agent_angle = agentSrc[agent_id].angle;

    let speed = 1.0;

    var new_agent: Agent;
    new_agent.position.x = speed * cos(agent_angle) + agent_x;
    new_agent.position.y = speed * sin(agent_angle) + agent_y;
    new_agent.angle = agent_angle;
    
    if (new_agent.position.x < 0.0
        || new_agent.position.y < 0.0
        || u32(new_agent.position.x) > uniforms.dimensions.x
        || u32(new_agent.position.y) > uniforms.dimensions.y) {
        new_agent.position = agentSrc[agent_id].position;
        new_agent.angle = hash_2d(new_agent.position + new_agent.angle) * 6.28;
    }

    agentDest[agent_id] = new_agent;
    textureStore(agent_texture,
        vec2<i32>(i32(new_agent.position.x), i32(new_agent.position.y)),
        vec4<f32>(1.0, 1.0, 1.0, 1.0)
    );
    textureStore(env_texture,
        vec2<i32>(i32(new_agent.position.x), i32(new_agent.position.y)),
        vec4<f32>(1.0, 1.0, 1.0, 1.0)
    );
}
