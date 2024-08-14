#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}

const PI: f32 = 3.141592653589793;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) i_prime: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let elapsed_seconds = 0.0; // TODO: Pass in elapsed time

    let prime = f32(vertex.i_prime);

    let scale = 0.2 + 1.0 * (sin(2.0 * elapsed_seconds + prime * 0.1) + 1.0) / 2.0;
    let color = vec4<f32>(
        1.5 + 0.5 * (sin(1.0 * elapsed_seconds + prime * 0.0008) + 1.0) / 2.0,
        1.5,
        1.5 + pow(1.0 / prime, 0.2),
        1.0,
    );

    let angle = prime % (PI * 2.0);
    let position = scale * vertex.position + vec3<f32>(
        prime * cos(angle - 0.002 * elapsed_seconds) / 50.0,
        prime * sin(angle - 0.002 * elapsed_seconds) / 50.0,
        0.0,
    );

    var out: VertexOutput;
    out.clip_position = mesh2d_position_local_to_clip(
        get_world_from_local(0u),
        vec4<f32>(position, 1.0)
    );
    out.color = color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
