#version 450

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
    vec4 light_dir;
} ubo;

layout(location = 0) in vec3 frag_normal;
layout(location = 1) in vec3 frag_color;
layout(location = 2) in float frag_view_depth;

layout(location = 0) out vec4 out_color;

const vec3 kSkyColor = vec3(0.53, 0.68, 0.85);
const float kFogStart = 140.0;
const float kFogEnd = 320.0;

void main() {
    vec3 normal = normalize(frag_normal);
    vec3 to_light = normalize(-ubo.light_dir.xyz);

    float diffuse = max(dot(normal, to_light), 0.0);
    float ambient = 0.35;
    vec3 lit = frag_color * (ambient + diffuse * 0.85);

    // Distance fog blends distant terrain into the sky.
    float fog = clamp((frag_view_depth - kFogStart) / (kFogEnd - kFogStart), 0.0, 1.0);
    vec3 color = mix(lit, kSkyColor, fog);

    // Gamma correction (linear -> sRGB approximation).
    color = pow(color, vec3(1.0 / 2.2));
    out_color = vec4(color, 1.0);
}
