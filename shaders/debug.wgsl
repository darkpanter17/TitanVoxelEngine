// Estructura Uniform que viene de Rust (Bind Group 0)
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) layer: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // Multiplicación de Matriz: Proyección * Vista * Posición
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    
    // Color temporal basado en la capa
    let r = f32(model.layer % 2u);
    let g = f32((model.layer + 1u) % 2u);
    out.color = vec3<f32>(r, g, 0.2);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}