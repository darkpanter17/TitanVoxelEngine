#![allow(dead_code)]
#![allow(unused_imports)]

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

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    // Calculate chunk world offset
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let voxel_id = chunk.get_voxel(x, y, z);
                if voxel_id == 0 {
                    continue; // Skip air
                }

                // Voxel ID maps to texture layer (ID 1 -> layer 0)
                let layer = (voxel_id - 1) as u32;

                let xi = x as i32;
                let yi = y as i32;
                let zi = z as i32;

                let xf = xi as f32 + offset_x;
                let yf = yi as f32 + offset_y;
                let zf = zi as f32 + offset_z;

                // +Y (Top)
                if chunk.get_voxel_safe(xi, yi + 1, zi) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [xf, yf + 1.0, zf + 1.0],
                            [xf + 1.0, yf + 1.0, zf + 1.0],
                            [xf + 1.0, yf + 1.0, zf],
                            [xf, yf + 1.0, zf],
                        ],
                        layer
                    );
                }

                // -Y (Bottom)
                if chunk.get_voxel_safe(xi, yi - 1, zi) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [xf, yf, zf],
                            [xf + 1.0, yf, zf],
                            [xf + 1.0, yf, zf + 1.0],
                            [xf, yf, zf + 1.0],
                        ],
                        layer
                    );
                }

                // +Z (Front)
                if chunk.get_voxel_safe(xi, yi, zi + 1) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [xf, yf, zf + 1.0],
                            [xf + 1.0, yf, zf + 1.0],
                            [xf + 1.0, yf + 1.0, zf + 1.0],
                            [xf, yf + 1.0, zf + 1.0],
                        ],
                        layer
                    );
                }

                // -Z (Back)
                if chunk.get_voxel_safe(xi, yi, zi - 1) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [xf + 1.0, yf, zf],
                            [xf, yf, zf],
                            [xf, yf + 1.0, zf],
                            [xf + 1.0, yf + 1.0, zf],
                        ],
                        layer
                    );
                }

                // +X (Right)
                if chunk.get_voxel_safe(xi + 1, yi, zi) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [xf + 1.0, yf, zf + 1.0],
                            [xf + 1.0, yf, zf],
                            [xf + 1.0, yf + 1.0, zf],
                            [xf + 1.0, yf + 1.0, zf + 1.0],
                        ],
                        layer
                    );
                }

                // -X (Left)
                if chunk.get_voxel_safe(xi - 1, yi, zi) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [xf, yf, zf],
                            [xf, yf, zf + 1.0],
                            [xf, yf + 1.0, zf + 1.0],
                            [xf, yf + 1.0, zf],
                        ],
                        layer
                    );
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(
    verts: &mut Vec<Vertex>,
    inds: &mut Vec<u32>,
    count: &mut u32,
    pos: [[f32; 3]; 4],
    layer: u32,
) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, 1.0], layer }); // Bottom Left
    verts.push(Vertex { pos: pos[1], uv: [1.0, 1.0], layer }); // Bottom Right
    verts.push(Vertex { pos: pos[2], uv: [1.0, 0.0], layer }); // Top Right
    verts.push(Vertex { pos: pos[3], uv: [0.0, 0.0], layer }); // Top Left

    // Winding order: 0, 1, 2,  2, 3, 0 (CCW)
    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count + 2, *count + 3, *count]);
    *count += 4;
}