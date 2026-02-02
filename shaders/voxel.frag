#version 450 core

layout(location = 0) in vec2 v_Uv;
layout(location = 1) in flat uint v_Layer;

layout(location = 0) out vec4 f_Color;

// Aquí está la magia: Un array de texturas, no una textura simple
uniform sampler2DArray u_TextureArray;

void main() {
    // texture() toma vec3(u, v, layer)
    // Buscamos el píxel en la capa correspondiente al ID del bloque
    vec4 tex_color = texture(u_TextureArray, vec3(v_Uv, float(v_Layer)));
    
    if(tex_color.a < 0.1) discard; // Transparencia (alpha testing)
    
    f_Color = tex_color;
}