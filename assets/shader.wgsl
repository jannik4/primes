#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}

const PI: f32 = 3.141592653589793;

struct Globals {
    elapsed_seconds: f32,
    zoom: f32,
    _padding0: u32,
    _padding1: u32,
}

@group(2) @binding(0)
var<uniform> globals: Globals;

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

fn gamma_function(value: f32) -> f32 {
    if value <= 0.0 {
        return value;
    }
    if value <= 0.04045 {
        return value / 12.92; // linear falloff in dark values
    }
    return pow((value + 0.055) / 1.055, 2.4); // gamma curve in other area
}

fn zoom_scale() -> f32 {
    if globals.zoom > 0.0 {
        return 1.0 / pow(1.5, globals.zoom);
    }

    return 1.0 / pow(1.75, globals.zoom);
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let time = globals.elapsed_seconds;

    let prime = f32(vertex.i_prime);

    let scale = (0.2 + (sin(2.0 * time + prime * 0.1) + 1.0) / 2.0) * zoom_scale(); // * (1.0 + 0.00002 * prime)
    let color = vec4<f32>(
        gamma_function(1.5 + 0.5 * (sin(1.0 * time + prime * 0.0008) + 1.0) / 2.0),
        gamma_function(1.5),
        gamma_function(1.5 + pow(1.0 / prime, 0.2)),
        1.0,
    );

    let angle = prime % (PI * 2.0);
    let position = scale * vertex.position + vec3<f32>(
        prime * cos(angle - 0.002 * time) / 50.0,
        prime * sin(angle - 0.002 * time) / 50.0,
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
