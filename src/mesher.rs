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
    
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                 let voxel = chunk.get_voxel(x, y, z);
                 if voxel == 0 { continue; }

                 let layer = (voxel - 1) as u32; // ID 1 -> Layer 0

                 let xf = offset_x + x as f32;
                 let yf = offset_y + y as f32;
                 let zf = offset_z + z as f32;

                 // Cara Superior (Y+)
                 if chunk.get_voxel_safe(x as i32, (y + 1) as i32, z as i32) == 0 {
                     push_face(&mut vertices, &mut indices, &mut index_count,
                               [[xf, yf+1.0, zf], [xf, yf+1.0, zf+1.0], [xf+1.0, yf+1.0, zf+1.0], [xf+1.0, yf+1.0, zf]],
                               [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]], layer);
                 }
                 // Cara Inferior (Y-)
                 if chunk.get_voxel_safe(x as i32, (y as i32) - 1, z as i32) == 0 {
                     push_face(&mut vertices, &mut indices, &mut index_count,
                               [[xf, yf, zf+1.0], [xf, yf, zf], [xf+1.0, yf, zf], [xf+1.0, yf, zf+1.0]],
                               [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]], layer);
                 }
                 // Cara Derecha (X+)
                 if chunk.get_voxel_safe((x + 1) as i32, y as i32, z as i32) == 0 {
                     push_face(&mut vertices, &mut indices, &mut index_count,
                               [[xf+1.0, yf, zf], [xf+1.0, yf+1.0, zf], [xf+1.0, yf+1.0, zf+1.0], [xf+1.0, yf, zf+1.0]],
                               [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]], layer);
                 }
                 // Cara Izquierda (X-)
                 if chunk.get_voxel_safe((x as i32) - 1, y as i32, z as i32) == 0 {
                     push_face(&mut vertices, &mut indices, &mut index_count,
                               [[xf, yf, zf+1.0], [xf, yf+1.0, zf+1.0], [xf, yf+1.0, zf], [xf, yf, zf]],
                               [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]], layer);
                 }
                 // Cara Frontal (Z+)
                 if chunk.get_voxel_safe(x as i32, y as i32, (z + 1) as i32) == 0 {
                     push_face(&mut vertices, &mut indices, &mut index_count,
                               [[xf+1.0, yf, zf+1.0], [xf+1.0, yf+1.0, zf+1.0], [xf, yf+1.0, zf+1.0], [xf, yf, zf+1.0]],
                               [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]], layer);
                 }
                 // Cara Trasera (Z-)
                 if chunk.get_voxel_safe(x as i32, y as i32, (z as i32) - 1) == 0 {
                     push_face(&mut vertices, &mut indices, &mut index_count,
                               [[xf, yf, zf], [xf, yf+1.0, zf], [xf+1.0, yf+1.0, zf], [xf+1.0, yf, zf]],
                               [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]], layer);
                 }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             pos: [[f32; 3]; 4], uv: [[f32; 2]; 4], layer: u32) {
    verts.push(Vertex { pos: pos[0], uv: uv[0], layer });
    verts.push(Vertex { pos: pos[1], uv: uv[1], layer });
    verts.push(Vertex { pos: pos[2], uv: uv[2], layer });
    verts.push(Vertex { pos: pos[3], uv: uv[3], layer });

    // Counter-clockwise winding: 0, 1, 2, 2, 3, 0
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}