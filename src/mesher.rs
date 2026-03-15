use crate::chunk::{Chunk, CHUNK_HEIGHT, CHUNK_SIZE};

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
                wgpu::VertexAttribute {
                    // pos
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    // uv
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    // layer
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

    for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let voxel = chunk.get_voxel(x, y, z);
                if voxel == 0 {
                    continue;
                }

                let layer = (voxel - 1) as u32;

                let xf = x as f32;
                let yf = y as f32;
                let zf = z as f32;

                let check = |dx: isize, dy: isize, dz: isize| -> bool {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    let nz = z as isize + dz;

                    if nx < 0 || ny < 0 || nz < 0 || nx >= CHUNK_SIZE as isize || ny >= CHUNK_HEIGHT as isize || nz >= CHUNK_SIZE as isize {
                        return true;
                    }
                    chunk.get_voxel(nx as usize, ny as usize, nz as usize) == 0
                };

                // Top (Y+)
                if check(0, 1, 0) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [xf, yf + 1.0, zf], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], layer);
                }
                // Bottom (Y-)
                if check(0, -1, 0) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [xf, yf, zf], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], layer);
                }
                // Left (X-)
                if check(-1, 0, 0) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [xf, yf, zf], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], layer);
                }
                // Right (X+)
                if check(1, 0, 0) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [xf + 1.0, yf, zf + 1.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0], layer);
                }
                // Front (Z+)
                if check(0, 0, 1) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [xf, yf, zf + 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], layer);
                }
                // Back (Z-)
                if check(0, 0, -1) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [xf + 1.0, yf, zf], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], layer);
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
    origin: [f32; 3],
    right: [f32; 3],
    up: [f32; 3],
    layer: u32,
) {
    verts.push(Vertex { pos: origin, uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: [origin[0] + right[0], origin[1] + right[1], origin[2] + right[2]], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: [origin[0] + right[0] + up[0], origin[1] + right[1] + up[1], origin[2] + right[2] + up[2]], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: [origin[0] + up[0], origin[1] + up[1], origin[2] + up[2]], uv: [0.0, 0.0], layer });

    // CCW Winding order: 0, 1, 2, 2, 3, 0
    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count + 2, *count + 3, *count]);
    *count += 4;
}
