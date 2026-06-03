#version 450

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
    vec4 light_dir;
} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec3 in_color;

layout(location = 0) out vec3 frag_normal;
layout(location = 1) out vec3 frag_color;
layout(location = 2) out float frag_view_depth;

void main() {
    // Vertices are baked in world space, so the model matrix is identity.
    vec4 world_pos = vec4(in_position, 1.0);
    vec4 view_pos = ubo.view * world_pos;
    gl_Position = ubo.proj * view_pos;

    frag_normal = in_normal;
    frag_color = in_color;
    frag_view_depth = -view_pos.z; // distance from camera for fog
}
