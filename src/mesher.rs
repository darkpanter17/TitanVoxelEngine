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
fn get_voxel_safe(chunk: &Chunk, x: i32, y: i32, z: i32) -> u16 {
    if x < 0 || x >= CHUNK_SIZE as i32 ||
       y < 0 || y >= CHUNK_HEIGHT as i32 ||
       z < 0 || z >= CHUNK_SIZE as i32 {
        return 0; // Assume air out of bounds
    }
    chunk.get_voxel(x as usize, y as usize, z as usize)
}

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
                let block_id = chunk.get_voxel(x, y, z);
                if block_id == 0 {
                    continue; // Skip air
                }

                let layer = block_id as u32 - 1; // Map 1->0, 2->1, etc

                let xi = x as i32;
                let yi = y as i32;
                let zi = z as i32;

                let px = offset_x + x as f32;
                let py = offset_y + y as f32;
                let pz = offset_z + z as f32;
                let p1 = px + 1.0;
                let p2 = py + 1.0;
                let p3 = pz + 1.0;

                // Top (Y+)
                if get_voxel_safe(chunk, xi, yi + 1, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[px, p2, p3], [p1, p2, p3], [p1, p2, pz], [px, p2, pz]], layer);
                }
                // Bottom (Y-)
                if get_voxel_safe(chunk, xi, yi - 1, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[px, py, pz], [p1, py, pz], [p1, py, p3], [px, py, p3]], layer);
                }
                // Front (Z+)
                if get_voxel_safe(chunk, xi, yi, zi + 1) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[px, py, p3], [p1, py, p3], [p1, p2, p3], [px, p2, p3]], layer);
                }
                // Back (Z-)
                if get_voxel_safe(chunk, xi, yi, zi - 1) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[px, p2, pz], [p1, p2, pz], [p1, py, pz], [px, py, pz]], layer);
                }
                // Right (X+)
                if get_voxel_safe(chunk, xi + 1, yi, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[p1, py, pz], [p1, p2, pz], [p1, p2, p3], [p1, py, p3]], layer);
                }
                // Left (X-)
                if get_voxel_safe(chunk, xi - 1, yi, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[px, py, p3], [px, p2, p3], [px, p2, pz], [px, py, pz]], layer);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_quad(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, 
             positions: [[f32; 3]; 4], layer: u32) {
    verts.push(Vertex { pos: positions[0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: positions[1], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: positions[2], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: positions[3], uv: [0.0, 0.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}