use crate::chunk::{Chunk, CHUNK_SIZE, CHUNK_HEIGHT};

// Derivamos Pod y Zeroable para que bytemuck pueda copiar esto como bytes puros
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub layer: u32,
}

impl Vertex {
    // Le explicamos a WGPU cómo leer este struct
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { // pos
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { // uv
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute { // layer
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

// Genera geometría de todas las caras con culling
pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let voxel_id = chunk.get_voxel(x, y, z);
                if voxel_id == 0 { continue; }

                let xi = x as i32;
                let yi = y as i32;
                let zi = z as i32;

                let xf = xi as f32 + offset_x;
                let yf = yi as f32 + offset_y;
                let zf = zi as f32 + offset_z;

                // Mapeo básico de textura:
                // Asumimos textura 256x256, con sprites 16x16, para un atlas.
                // layer es equivalente a "y" en el atlas si lo tratáramos como array
                // voxel_id = 1 (Dirt)
                // voxel_id = 2 (Grass)
                // voxel_id = 3 (Stone)
                // Usamos el id de voxel como la capa, o en este caso, lo mapeamos al atlas
                // Grass: top=32, side=16, bottom=0 (basado en fogleman/Craft: dirty grass top)
                // Dirt: 0
                // Stone: 1
                let (top_id, side_id, bottom_id) = match voxel_id {
                    1 => (0, 0, 0),
                    2 => (32, 16, 0),
                    3 => (1, 1, 1),
                    _ => (0, 0, 0),
                };

                let uv_p00 = [0.0, 0.0];
                let uv_p10 = [1.0, 0.0];
                let uv_p11 = [1.0, 1.0];
                let uv_p01 = [0.0, 1.0];

                // Y+ (Top) - CCW
                if chunk.get_voxel_safe(xi, yi + 1, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf], [xf, yf + 1.0, zf]],
                        [uv_p00, uv_p10, uv_p11, uv_p01], top_id);
                }
                // Y- (Bottom) - CCW
                if chunk.get_voxel_safe(xi, yi - 1, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf, yf, zf], [xf + 1.0, yf, zf], [xf + 1.0, yf, zf + 1.0], [xf, yf, zf + 1.0]],
                        [uv_p00, uv_p10, uv_p11, uv_p01], bottom_id);
                }
                // X+ (Right) - CCW
                if chunk.get_voxel_safe(xi + 1, yi, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf, zf], [xf + 1.0, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf + 1.0]],
                        [uv_p00, uv_p10, uv_p11, uv_p01], side_id);
                }
                // X- (Left) - CCW
                if chunk.get_voxel_safe(xi - 1, yi, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf, yf, zf], [xf, yf, zf + 1.0], [xf, yf + 1.0, zf + 1.0], [xf, yf + 1.0, zf]],
                        [uv_p00, uv_p10, uv_p11, uv_p01], side_id);
                }
                // Z+ (Front) - CCW
                if chunk.get_voxel_safe(xi, yi, zi + 1) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf, yf, zf + 1.0], [xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf, yf + 1.0, zf + 1.0]],
                        [uv_p00, uv_p10, uv_p11, uv_p01], side_id);
                }
                // Z- (Back) - CCW
                if chunk.get_voxel_safe(xi, yi, zi - 1) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf + 1.0, yf, zf], [xf, yf, zf], [xf, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf]],
                        [uv_p00, uv_p10, uv_p11, uv_p01], side_id);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_quad(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, 
             pos: [[f32; 3]; 4], uvs: [[f32; 2]; 4], layer: u32) {
    verts.push(Vertex { pos: pos[0], uv: uvs[0], layer });
    verts.push(Vertex { pos: pos[1], uv: uvs[1], layer });
    verts.push(Vertex { pos: pos[2], uv: uvs[2], layer });
    verts.push(Vertex { pos: pos[3], uv: uvs[3], layer });
    // 0,1,2 and 0,2,3 for CCW
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count, *count+2, *count+3]);
    *count += 4;
}