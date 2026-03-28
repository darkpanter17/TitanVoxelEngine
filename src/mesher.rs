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
    
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                let voxel = chunk.get_voxel(x, y, z);
                if voxel == 0 { continue; }

                let layer = (voxel - 1) as u32; // ID 1 -> layer 0
                let wx = x as f32 + offset_x;
                let wy = y as f32 + offset_y;
                let wz = z as f32 + offset_z;
                let xi = x as i32;
                let yi = y as i32;
                let zi = z as i32;

                // Cara Frontal (Z+)
                if chunk.get_voxel_safe(xi, yi, zi + 1) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[wx, wy, wz+1.0], [wx+1.0, wy, wz+1.0], [wx+1.0, wy+1.0, wz+1.0], [wx, wy+1.0, wz+1.0]],
                        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], layer);
                }
                // Cara Trasera (Z-)
                if chunk.get_voxel_safe(xi, yi, zi - 1) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[wx+1.0, wy, wz], [wx, wy, wz], [wx, wy+1.0, wz], [wx+1.0, wy+1.0, wz]],
                        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], layer);
                }
                // Cara Izquierda (X-)
                if chunk.get_voxel_safe(xi - 1, yi, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[wx, wy, wz], [wx, wy, wz+1.0], [wx, wy+1.0, wz+1.0], [wx, wy+1.0, wz]],
                        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], layer);
                }
                // Cara Derecha (X+)
                if chunk.get_voxel_safe(xi + 1, yi, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[wx+1.0, wy, wz+1.0], [wx+1.0, wy, wz], [wx+1.0, wy+1.0, wz], [wx+1.0, wy+1.0, wz+1.0]],
                        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], layer);
                }
                // Cara Inferior (Y-)
                if chunk.get_voxel_safe(xi, yi - 1, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[wx, wy, wz], [wx+1.0, wy, wz], [wx+1.0, wy, wz+1.0], [wx, wy, wz+1.0]],
                        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], layer);
                }
                // Cara Superior (Y+)
                if chunk.get_voxel_safe(xi, yi + 1, zi) == 0 {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[wx, wy+1.0, wz+1.0], [wx+1.0, wy+1.0, wz+1.0], [wx+1.0, wy+1.0, wz], [wx, wy+1.0, wz]],
                        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], layer);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_quad(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, 
             pos: [[f32; 3]; 4], uv: [[f32; 2]; 4], layer: u32) {
    verts.push(Vertex { pos: pos[0], uv: uv[0], layer });
    verts.push(Vertex { pos: pos[1], uv: uv[1], layer });
    verts.push(Vertex { pos: pos[2], uv: uv[2], layer });
    verts.push(Vertex { pos: pos[3], uv: uv[3], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}