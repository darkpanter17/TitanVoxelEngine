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

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;

    let ox = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let oy = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let oz = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let voxel = chunk.get_voxel(x, y, z);
                if voxel == 0 {
                    continue;
                }

                let layer = (voxel - 1) as u32;

                let xi = x as i32;
                let yi = y as i32;
                let zi = z as i32;

                let px = ox + x as f32;
                let py = oy + y as f32;
                let pz = oz + z as f32;

                // Right (+X)
                if chunk.get_voxel_safe(xi + 1, yi, zi) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [[px + 1.0, py, pz + 1.0], [px + 1.0, py, pz], [px + 1.0, py + 1.0, pz], [px + 1.0, py + 1.0, pz + 1.0]],
                              layer);
                }
                // Left (-X)
                if chunk.get_voxel_safe(xi - 1, yi, zi) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [[px, py, pz], [px, py, pz + 1.0], [px, py + 1.0, pz + 1.0], [px, py + 1.0, pz]],
                              layer);
                }
                // Top (+Y)
                if chunk.get_voxel_safe(xi, yi + 1, zi) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [[px, py + 1.0, pz + 1.0], [px + 1.0, py + 1.0, pz + 1.0], [px + 1.0, py + 1.0, pz], [px, py + 1.0, pz]],
                              layer);
                }
                // Bottom (-Y)
                if chunk.get_voxel_safe(xi, yi - 1, zi) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [[px, py, pz], [px + 1.0, py, pz], [px + 1.0, py, pz + 1.0], [px, py, pz + 1.0]],
                              layer);
                }
                // Front (+Z)
                if chunk.get_voxel_safe(xi, yi, zi + 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [[px, py, pz + 1.0], [px + 1.0, py, pz + 1.0], [px + 1.0, py + 1.0, pz + 1.0], [px, py + 1.0, pz + 1.0]],
                              layer);
                }
                // Back (-Z)
                if chunk.get_voxel_safe(xi, yi, zi - 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [[px + 1.0, py, pz], [px, py, pz], [px, py + 1.0, pz], [px + 1.0, py + 1.0, pz]],
                              layer);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             corners: [[f32; 3]; 4], layer: u32) {
    verts.push(Vertex { pos: corners[0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: corners[1], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: corners[2], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: corners[3], uv: [0.0, 0.0], layer });
    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count + 2, *count + 3, *count]);
    *count += 4;
}