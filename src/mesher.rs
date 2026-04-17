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

// (Mantenemos la lógica de Greedy Meshing igual, solo cambia el Vertex struct arriba)
pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let ox = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let oy = (chunk.position.y * CHUNK_HEIGHT as i32) as f32; // Normally 0 if y=0 chunks
    let oz = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let voxel = chunk.get_voxel_safe(x as i32, y as i32, z as i32);
                if voxel == 0 {
                    continue;
                }

                let layer = (voxel - 1) as u32;

                let xf = ox + x as f32;
                let yf = oy + y as f32;
                let zf = oz + z as f32;

                // Culling: Only draw exposed faces (Neighbor is air)

                // Top (+Y)
                if chunk.get_voxel_safe(x as i32, y as i32 + 1, z as i32) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf + 1.0], [xf, yf + 1.0, zf + 1.0]], layer);
                }

                // Bottom (-Y)
                if chunk.get_voxel_safe(x as i32, y as i32 - 1, z as i32) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf, yf, zf + 1.0], [xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf, zf], [xf, yf, zf]], layer);
                }

                // Left (-X)
                if chunk.get_voxel_safe(x as i32 - 1, y as i32, z as i32) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf, yf, zf], [xf, yf, zf + 1.0], [xf, yf + 1.0, zf + 1.0], [xf, yf + 1.0, zf]], layer);
                }

                // Right (+X)
                if chunk.get_voxel_safe(x as i32 + 1, y as i32, z as i32) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf, zf], [xf + 1.0, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf + 1.0]], layer);
                }

                // Back (-Z)
                if chunk.get_voxel_safe(x as i32, y as i32, z as i32 - 1) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf + 1.0, yf, zf], [xf, yf, zf], [xf, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf]], layer);
                }

                // Front (+Z)
                if chunk.get_voxel_safe(x as i32, y as i32, z as i32 + 1) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[xf, yf, zf + 1.0], [xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf, yf + 1.0, zf + 1.0]], layer);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_quad(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             pos: [[f32; 3]; 4], layer: u32) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: pos[1], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: pos[2], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: pos[3], uv: [0.0, 1.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}