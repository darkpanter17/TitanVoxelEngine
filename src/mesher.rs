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
    
    let chunk_x_offset = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let chunk_y_offset = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let chunk_z_offset = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                let voxel = chunk.get_voxel(x, y, z);
                if voxel == 0 {
                    continue;
                }

                let layer = voxel as u32 - 1; // Map Block ID 1 -> Layer 0

                let cx = x as i32;
                let cy = y as i32;
                let cz = z as i32;

                let xf = chunk_x_offset + x as f32;
                let yf = chunk_y_offset + y as f32;
                let zf = chunk_z_offset + z as f32;

                // X+ (Right)
                if chunk.get_voxel_safe(cx + 1, cy, cz) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, [
                        [xf + 1.0, yf      , zf + 1.0],
                        [xf + 1.0, yf      , zf      ],
                        [xf + 1.0, yf + 1.0, zf      ],
                        [xf + 1.0, yf + 1.0, zf + 1.0],
                    ], layer);
                }

                // X- (Left)
                if chunk.get_voxel_safe(cx - 1, cy, cz) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, [
                        [xf, yf      , zf      ],
                        [xf, yf      , zf + 1.0],
                        [xf, yf + 1.0, zf + 1.0],
                        [xf, yf + 1.0, zf      ],
                    ], layer);
                }

                // Y+ (Top)
                if chunk.get_voxel_safe(cx, cy + 1, cz) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, [
                        [xf      , yf + 1.0, zf + 1.0],
                        [xf + 1.0, yf + 1.0, zf + 1.0],
                        [xf + 1.0, yf + 1.0, zf      ],
                        [xf      , yf + 1.0, zf      ],
                    ], layer);
                }

                // Y- (Bottom)
                if chunk.get_voxel_safe(cx, cy - 1, cz) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, [
                        [xf      , yf, zf      ],
                        [xf + 1.0, yf, zf      ],
                        [xf + 1.0, yf, zf + 1.0],
                        [xf      , yf, zf + 1.0],
                    ], layer);
                }

                // Z+ (Front)
                if chunk.get_voxel_safe(cx, cy, cz + 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, [
                        [xf      , yf      , zf + 1.0],
                        [xf + 1.0, yf      , zf + 1.0],
                        [xf + 1.0, yf + 1.0, zf + 1.0],
                        [xf      , yf + 1.0, zf + 1.0],
                    ], layer);
                }

                // Z- (Back)
                if chunk.get_voxel_safe(cx, cy, cz - 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, [
                        [xf + 1.0, yf      , zf      ],
                        [xf      , yf      , zf      ],
                        [xf      , yf + 1.0, zf      ],
                        [xf + 1.0, yf + 1.0, zf      ],
                    ], layer);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             pos: [[f32; 3]; 4], layer: u32) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: pos[1], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: pos[2], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: pos[3], uv: [0.0, 0.0], layer });

    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}