#version 450 core

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec2 a_Uv;
layout(location = 2) in uint a_Layer; // El ID del bloque (0-1000)

// Salidas para el Fragment Shader
layout(location = 0) out vec2 v_Uv;
// 'flat' es VITAL: evita que el ID se interpole (no queremos que el bloque 1 se mezcle con el 2)
layout(location = 1) out flat uint v_Layer; 

uniform mat4 u_ViewProj;
uniform vec3 u_ChunkPos;

void main() {
    v_Uv = a_Uv;
    v_Layer = a_Layer;
    
    // Calculamos posición final: Posición local + Posición del Chunk
    vec3 world_pos = a_Pos + u_ChunkPos;
    gl_Position = u_ViewProj * vec4(world_pos, 1.0);
}