struct Agent {
    position: vec2<f32>,
    angle: f32,
    turn_speed: f32,
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
@group(0) @binding(3) var<storage, read> env_src: array<EnvCell>;
@group(0) @binding(4) var<storage, read_write> env_dest: array<EnvCell>;

@group(1) @binding(0) var<uniform> uniforms: Uniforms;

@compute
@workgroup_size(8, 1, 1)
fn compute_main(
    in: ComputeInput,
) {
    let agent_id = in.global_id.x;
    let agent_pos = agent_src[agent_id].position;
    let agent_x = agent_pos.x;
    let agent_y = agent_pos.y;
    let agent_angle = agent_src[agent_id].angle;
    let agent_turn_speed = agent_src[agent_id].turn_speed;
    let agent_hash = hash_2d(agent_pos + agent_angle);

    let speed = 1.0;

    var new_agent: Agent;
    new_agent.position.x = speed * cos(agent_angle) + agent_x;
    new_agent.position.y = speed * sin(agent_angle) + agent_y;
    new_agent.angle = agent_angle + agent_turn_speed;
    new_agent.turn_speed = agent_turn_speed + ( 0.0005 * ( agent_hash - 0.5 ) );
    
    // Wall hit detection
    if (new_agent.position.x < 0.0
        || new_agent.position.y < 0.0
        || u32(new_agent.position.x) > uniforms.dimensions.x
        || u32(new_agent.position.y) > uniforms.dimensions.y) {
        new_agent.position = agent_pos;
        new_agent.angle = agent_hash * 6.28;
    }

    // Pheromone detection
    let detect_angle_spread = 1.0;
    let detect_distance = 6.0;
    let detect_influence = 0.1;
    let angle_left = agent_angle - detect_angle_spread;
    let angle_right = agent_angle + detect_angle_spread;
    let origin_left = vec2<i32>(
        i32( detect_distance * cos(angle_left) + agent_x ),
        i32( detect_distance * sin(angle_left) + agent_y ),
    );
    let origin_straight = vec2<i32>(
        i32( detect_distance * cos(agent_angle) + agent_x ),
        i32( detect_distance * sin(agent_angle) + agent_y ),
    );
    let origin_right = vec2<i32>(
        i32( detect_distance * cos(angle_right) + agent_x ),
        i32( detect_distance * sin(angle_right) + agent_y ),
    );
    var pheromones_left = 0.0;
    var pheromones_straight = 0.0;
    var pheromones_right = 0.0;
    for (var i: i32 = -2; i <= 2; i++) {
        for (var j: i32 = -2; j <= 2; j++) {
            let left_check = vec2<u32>(
                u32(origin_left.x + i),
                u32(origin_left.y + j),
            );
            let straight_check = vec2<u32>(
                u32(origin_straight.x + i),
                u32(origin_straight.y + j),
            );
            let right_check = vec2<u32>(
                u32(origin_right.x + i),
                u32(origin_right.y + j),
            );

            if (left_check.x >= uniforms.dimensions.x
                || left_check.y >= uniforms.dimensions.y
                || straight_check.x >= uniforms.dimensions.x
                || straight_check.y >= uniforms.dimensions.y
                || right_check.x >= uniforms.dimensions.x
                || right_check.y >= uniforms.dimensions.y) {
                continue;
            }
            pheromones_left += env_src[left_check.x * uniforms.dimensions.x + left_check.y].pheromone_level;
            pheromones_straight += env_src[straight_check.x * uniforms.dimensions.x + straight_check.y].pheromone_level;
            pheromones_right += env_src[right_check.x * uniforms.dimensions.x + right_check.y].pheromone_level;
        }
    }
    if (pheromones_left > pheromones_right && pheromones_left > pheromones_straight) {
        new_agent.angle -= detect_influence * detect_angle_spread;
    } else if (pheromones_right > pheromones_straight) {
        new_agent.angle += detect_influence * detect_angle_spread;
    }

    // Draw the new agent's data
    agent_dest[agent_id] = new_agent;
    let agent_index = u32(new_agent.position.x) * uniforms.dimensions.x + u32(new_agent.position.y);
    env_dest[agent_index].pheromone_level = 1.0;
    textureStore(agent_texture,
        vec2<i32>(i32(new_agent.position.x), i32(new_agent.position.y)),
        vec4<f32>(1.0, 1.0, 1.0, 1.0)
    );
}
