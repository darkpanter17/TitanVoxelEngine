#[allow(unused_imports)]
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
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let voxel_id = chunk.get_voxel(x, y, z);
                if voxel_id == 0 {
                    continue;
                }

                let layer = (voxel_id - 1) as u32;

                let ix = x as i32;
                let iy = y as i32;
                let iz = z as i32;

                let fx = x as f32 + chunk_x_offset;
                let fy = y as f32 + chunk_y_offset;
                let fz = z as f32 + chunk_z_offset;

                // Face directions: Top (+Y), Bottom (-Y), Front (+Z), Back (-Z), Right (+X), Left (-X)

                // Top (+Y)
                if chunk.get_voxel_safe(ix, iy + 1, iz) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [fx, fy + 1.0, fz + 1.0], // Top-left
                            [fx + 1.0, fy + 1.0, fz + 1.0], // Top-right
                            [fx + 1.0, fy + 1.0, fz], // Bottom-right
                            [fx, fy + 1.0, fz]  // Bottom-left
                        ],
                        layer
                    );
                }

                // Bottom (-Y)
                if chunk.get_voxel_safe(ix, iy - 1, iz) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [fx, fy, fz], // Top-left
                            [fx + 1.0, fy, fz], // Top-right
                            [fx + 1.0, fy, fz + 1.0], // Bottom-right
                            [fx, fy, fz + 1.0]  // Bottom-left
                        ],
                        layer
                    );
                }

                // Front (+Z)
                if chunk.get_voxel_safe(ix, iy, iz + 1) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [fx, fy + 1.0, fz + 1.0], // Top-left
                            [fx, fy, fz + 1.0], // Bottom-left
                            [fx + 1.0, fy, fz + 1.0], // Bottom-right
                            [fx + 1.0, fy + 1.0, fz + 1.0]  // Top-right
                        ],
                        layer
                    );
                }

                // Back (-Z)
                if chunk.get_voxel_safe(ix, iy, iz - 1) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [fx + 1.0, fy + 1.0, fz], // Top-left
                            [fx + 1.0, fy, fz], // Bottom-left
                            [fx, fy, fz], // Bottom-right
                            [fx, fy + 1.0, fz]  // Top-right
                        ],
                        layer
                    );
                }

                // Right (+X)
                if chunk.get_voxel_safe(ix + 1, iy, iz) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [fx + 1.0, fy + 1.0, fz + 1.0], // Top-left
                            [fx + 1.0, fy, fz + 1.0], // Bottom-left
                            [fx + 1.0, fy, fz], // Bottom-right
                            [fx + 1.0, fy + 1.0, fz]  // Top-right
                        ],
                        layer
                    );
                }

                // Left (-X)
                if chunk.get_voxel_safe(ix - 1, iy, iz) == 0 {
                    push_face(
                        &mut vertices, &mut indices, &mut index_count,
                        [
                            [fx, fy + 1.0, fz], // Top-left
                            [fx, fy, fz], // Bottom-left
                            [fx, fy, fz + 1.0], // Bottom-right
                            [fx, fy + 1.0, fz + 1.0]  // Top-right
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
    positions: [[f32; 3]; 4],
    layer: u32
) {
    verts.push(Vertex { pos: positions[0], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: positions[1], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: positions[2], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: positions[3], uv: [1.0, 0.0], layer });

    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}