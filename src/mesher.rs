use crate::chunk::{Chunk, CHUNK_SIZE};

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
// Helper to safely get voxel ID (returns 0 if out of bounds)
fn get_voxel_safe(chunk: &Chunk, x: i32, y: i32, z: i32) -> u16 {
    if x < 0 || y < 0 || z < 0 || x >= CHUNK_SIZE as i32 || y >= crate::chunk::CHUNK_HEIGHT as i32 || z >= CHUNK_SIZE as i32 {
        return 0; // Air
    }
    chunk.get_voxel(x as usize, y as usize, z as usize)
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * crate::chunk::CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..crate::chunk::CHUNK_HEIGHT {
                let id = chunk.get_voxel(x, y, z);
                if id == 0 {
                    continue;
                }

                let layer = if id > 0 { (id - 1) as u32 } else { 0 }; // Map Block ID to Texture Layer (e.g., 1 -> 0, 2 -> 1)

                let cx = x as i32;
                let cy = y as i32;
                let cz = z as i32;

                let px = offset_x + x as f32;
                let py = offset_y + y as f32;
                let pz = offset_z + z as f32;

                // Face Culling + Winding order counter-clockwise for outer-facing normal

                // Top (Y+)
                if get_voxel_safe(chunk, cx, cy + 1, cz) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, layer, [
                        [px, py + 1.0, pz + 1.0],
                        [px + 1.0, py + 1.0, pz + 1.0],
                        [px + 1.0, py + 1.0, pz],
                        [px, py + 1.0, pz],
                    ]);
                }

                // Bottom (Y-)
                if get_voxel_safe(chunk, cx, cy - 1, cz) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, layer, [
                        [px, py, pz],
                        [px + 1.0, py, pz],
                        [px + 1.0, py, pz + 1.0],
                        [px, py, pz + 1.0],
                    ]);
                }

                // Front (Z+)
                if get_voxel_safe(chunk, cx, cy, cz + 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, layer, [
                        [px, py, pz + 1.0],
                        [px + 1.0, py, pz + 1.0],
                        [px + 1.0, py + 1.0, pz + 1.0],
                        [px, py + 1.0, pz + 1.0],
                    ]);
                }

                // Back (Z-)
                if get_voxel_safe(chunk, cx, cy, cz - 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, layer, [
                        [px + 1.0, py, pz],
                        [px, py, pz],
                        [px, py + 1.0, pz],
                        [px + 1.0, py + 1.0, pz],
                    ]);
                }

                // Right (X+)
                if get_voxel_safe(chunk, cx + 1, cy, cz) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, layer, [
                        [px + 1.0, py, pz + 1.0],
                        [px + 1.0, py, pz],
                        [px + 1.0, py + 1.0, pz],
                        [px + 1.0, py + 1.0, pz + 1.0],
                    ]);
                }

                // Left (X-)
                if get_voxel_safe(chunk, cx - 1, cy, cz) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count, layer, [
                        [px, py, pz],
                        [px, py, pz + 1.0],
                        [px, py + 1.0, pz + 1.0],
                        [px, py + 1.0, pz],
                    ]);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, layer: u32, positions: [[f32; 3]; 4]) {
    verts.push(Vertex { pos: positions[0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: positions[1], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: positions[2], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: positions[3], uv: [0.0, 0.0], layer });

    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}