struct Agent {
    position : vec2<f32>,
    angle: f32,
};

struct ComputeInput {
@builtin(global_invocation_id) global_id: vec3<u32>,
};

@group(0) @binding(0) var<storage, read> agentSrc: array<Agent>;
@group(0) @binding(1) var<storage, read_write> agentDest: array<Agent>;
@group(0) @binding(2) var texture: texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(8, 1, 1)
fn compute_main(
    in: ComputeInput,
) {
    let agent_id = in.global_id.x;
    let agent_x = agentSrc[agent_id].position[0];
    let agent_y = agentSrc[agent_id].position[1];
    let agent_angle = agentSrc[agent_id].angle;

    agentDest[agent_id] = agentSrc[agent_id];
    textureStore(texture,
        vec2<i32>(i32(agent_x), i32(agent_y)),
        vec4<f32>(0.0, 1.0, 1.0, 1.0)
    );
}
