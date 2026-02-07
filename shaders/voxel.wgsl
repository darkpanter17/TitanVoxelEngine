// Bind Group 0: Camera
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera: CameraUniform;

// Bind Group 1: Textures
@group(1) @binding(0) var t_diffuse: texture_2d_array<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) layer: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(flat) layer: u32,
    @location(2) brightness: f32,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.uv = model.uv;
    out.layer = model.layer;

    // Simple Directional Lighting
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.5)); // Light from top-right-front
    let diffuse = max(dot(model.normal, light_dir), 0.0);
    let ambient = 0.3;
    out.brightness = ambient + (diffuse * 0.7); // 0.7 is diffuse intensity

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.uv, i32(in.layer));

    // Alpha testing
    if (tex_color.a < 0.1) {
        discard;
    }

    // Apply lighting
    let lit_color = vec3<f32>(tex_color.rgb * in.brightness);

    return vec4<f32>(lit_color, tex_color.a);
}
