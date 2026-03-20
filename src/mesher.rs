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

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                let voxel = chunk.get_voxel(x, y, z);
                if voxel != 0 {
                    let layer = (voxel - 1) as u32; // Offset -1 para la textura, por ejemplo ID 1 -> layer 0
                    let xf = x as f32;
                    let yf = y as f32;
                    let zf = z as f32;

                    // Cara Superior (Y+)
                    if y == CHUNK_HEIGHT - 1 || chunk.get_voxel(x, y + 1, z) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf], [xf, yf + 1.0, zf]],
                            layer
                        );
                    }

                    // Cara Inferior (Y-)
                    if y == 0 || chunk.get_voxel(x, y - 1, z) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf, yf, zf], [xf + 1.0, yf, zf], [xf + 1.0, yf, zf + 1.0], [xf, yf, zf + 1.0]],
                            layer
                        );
                    }

                    // Cara Izquierda (X-)
                    if x == 0 || chunk.get_voxel(x - 1, y, z) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf, yf, zf], [xf, yf, zf + 1.0], [xf, yf + 1.0, zf + 1.0], [xf, yf + 1.0, zf]],
                            layer
                        );
                    }

                    // Cara Derecha (X+)
                    if x == CHUNK_SIZE - 1 || chunk.get_voxel(x + 1, y, z) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf, zf], [xf + 1.0, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf + 1.0]],
                            layer
                        );
                    }

                    // Cara Frontal (Z+)
                    if z == CHUNK_SIZE - 1 || chunk.get_voxel(x, y, z + 1) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf, yf + 1.0, zf + 1.0], [xf, yf, zf + 1.0]],
                            layer
                        );
                    }

                    // Cara Trasera (Z-)
                    if z == 0 || chunk.get_voxel(x, y, z - 1) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf, yf, zf], [xf, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf], [xf + 1.0, yf, zf]],
                            layer
                        );
                    }
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_quad(
    verts: &mut Vec<Vertex>,
    inds: &mut Vec<u32>,
    count: &mut u32,
    positions: [[f32; 3]; 4],
    layer: u32,
) {
    verts.push(Vertex { pos: positions[0], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: positions[1], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: positions[2], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: positions[3], uv: [0.0, 1.0], layer });
    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count + 2, *count + 3, *count]);
    *count += 4;
}