#import bevy_pbr::forward_io::VertexOutput

@group(0) @binding(0) var<uniform> time: f32;
@group(0) @binding(1) var<uniform> colour: vec4<f32>;
@group(0) @binding(2) var<uniform> refraction: f32;
@group(0) @binding(3) var<uniform> absorbance: f32,
@group(0) @binding(4) var<uniform> turbulence: f32,

@vertex
fn vertex() -> VertexOutput {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}