use crate::chunk::{Chunk, CHUNK_SIZE, CHUNK_HEIGHT};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub layer: u32,
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
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

fn get_voxel_safe(chunk: &Chunk, x: i32, y: i32, z: i32) -> u16 {
    if x < 0 || x >= CHUNK_SIZE as i32 || y < 0 || y >= CHUNK_HEIGHT as i32 || z < 0 || z >= CHUNK_SIZE as i32 {
        return 0; // Return air if out of bounds (culling against chunk boundaries for now)
    }
    chunk.get_voxel(x as usize, y as usize, z as usize)
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let chunk_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let chunk_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let chunk_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                let voxel = chunk.get_voxel(x, y, z);
                if voxel == 0 {
                    continue;
                }

                let layer = voxel as u32 - 1; // Map block ID 1 to layer 0, etc.
                let wx = chunk_x + x as f32;
                let wy = chunk_y + y as f32;
                let wz = chunk_z + z as f32;

                let xi = x as i32;
                let yi = y as i32;
                let zi = z as i32;

                // Right (+X)
                if get_voxel_safe(chunk, xi + 1, yi, zi) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [[wx + 1.0, wy, wz + 1.0], [wx + 1.0, wy, wz], [wx + 1.0, wy + 1.0, wz], [wx + 1.0, wy + 1.0, wz + 1.0]],
                        layer);
                }
                // Left (-X)
                if get_voxel_safe(chunk, xi - 1, yi, zi) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [[wx, wy, wz], [wx, wy, wz + 1.0], [wx, wy + 1.0, wz + 1.0], [wx, wy + 1.0, wz]],
                        layer);
                }
                // Top (+Y)
                if get_voxel_safe(chunk, xi, yi + 1, zi) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [[wx, wy + 1.0, wz + 1.0], [wx + 1.0, wy + 1.0, wz + 1.0], [wx + 1.0, wy + 1.0, wz], [wx, wy + 1.0, wz]],
                        layer);
                }
                // Bottom (-Y)
                if get_voxel_safe(chunk, xi, yi - 1, zi) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [[wx, wy, wz], [wx + 1.0, wy, wz], [wx + 1.0, wy, wz + 1.0], [wx, wy, wz + 1.0]],
                        layer);
                }
                // Front (+Z)
                if get_voxel_safe(chunk, xi, yi, zi + 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [[wx, wy, wz + 1.0], [wx + 1.0, wy, wz + 1.0], [wx + 1.0, wy + 1.0, wz + 1.0], [wx, wy + 1.0, wz + 1.0]],
                        layer);
                }
                // Back (-Z)
                if get_voxel_safe(chunk, xi, yi, zi - 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [[wx + 1.0, wy, wz], [wx, wy, wz], [wx, wy + 1.0, wz], [wx + 1.0, wy + 1.0, wz]],
                        layer);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, pos: [[f32; 3]; 4], layer: u32) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: pos[1], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: pos[2], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: pos[3], uv: [0.0, 0.0], layer });

    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count + 2, *count + 3, *count]);
    *count += 4;
}
