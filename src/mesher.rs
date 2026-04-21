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
        return 0; // Return air if out of bounds for chunk-local meshing.
                  // For a multi-chunk engine, this would check neighbor chunks.
    }
    chunk.get_voxel(x as usize, y as usize, z as usize)
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let chunk_world_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let chunk_world_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let chunk_world_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE as i32 {
        for z in 0..CHUNK_SIZE as i32 {
            for y in 0..CHUNK_HEIGHT as i32 {
                 let voxel = chunk.get_voxel(x as usize, y as usize, z as usize);
                 if voxel != 0 {
                    let layer = voxel as u32 - 1; // ID 1 -> layer 0

                    let wx = chunk_world_x + x as f32;
                    let wy = chunk_world_y + y as f32;
                    let wz = chunk_world_z + z as f32;

                    // Top (Y+)
                    if get_voxel_safe(chunk, x, y + 1, z) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [[wx, wy + 1.0, wz], [wx + 1.0, wy + 1.0, wz], [wx + 1.0, wy + 1.0, wz + 1.0], [wx, wy + 1.0, wz + 1.0]], layer);
                    }
                    // Bottom (Y-)
                    if get_voxel_safe(chunk, x, y - 1, z) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [[wx + 1.0, wy, wz], [wx, wy, wz], [wx, wy, wz + 1.0], [wx + 1.0, wy, wz + 1.0]], layer);
                    }
                    // Left (X-)
                    if get_voxel_safe(chunk, x - 1, y, z) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [[wx, wy, wz + 1.0], [wx, wy, wz], [wx, wy + 1.0, wz], [wx, wy + 1.0, wz + 1.0]], layer);
                    }
                    // Right (X+)
                    if get_voxel_safe(chunk, x + 1, y, z) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [[wx + 1.0, wy, wz], [wx + 1.0, wy, wz + 1.0], [wx + 1.0, wy + 1.0, wz + 1.0], [wx + 1.0, wy + 1.0, wz]], layer);
                    }
                    // Front (Z+)
                    if get_voxel_safe(chunk, x, y, z + 1) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [[wx + 1.0, wy, wz + 1.0], [wx, wy, wz + 1.0], [wx, wy + 1.0, wz + 1.0], [wx + 1.0, wy + 1.0, wz + 1.0]], layer);
                    }
                    // Back (Z-)
                    if get_voxel_safe(chunk, x, y, z - 1) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [[wx, wy, wz], [wx + 1.0, wy, wz], [wx + 1.0, wy + 1.0, wz], [wx, wy + 1.0, wz]], layer);
                    }
                 }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             coords: [[f32; 3]; 4], layer: u32) {
    verts.push(Vertex { pos: coords[0], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: coords[1], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: coords[2], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: coords[3], uv: [0.0, 1.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}
