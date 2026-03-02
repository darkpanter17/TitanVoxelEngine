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

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let voxel = chunk.get_voxel(x, y, z);
                if voxel == 0 {
                    continue;
                }

                let layer = (voxel - 1) as u32;

                let xf = x as f32;
                let yf = y as f32;
                let zf = z as f32;

                // +Y Top
                if y == CHUNK_HEIGHT - 1 || chunk.get_voxel(x, y + 1, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [xf, yf + 1.0, zf], [xf, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf], layer);
                }
                // -Y Bottom
                if y == 0 || chunk.get_voxel(x, y - 1, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [xf, yf, zf + 1.0], [xf, yf, zf], [xf + 1.0, yf, zf], [xf + 1.0, yf, zf + 1.0], layer);
                }
                // +X Right
                if x == CHUNK_SIZE - 1 || chunk.get_voxel(x + 1, y, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf], [xf + 1.0, yf, zf], layer);
                }
                // -X Left
                if x == 0 || chunk.get_voxel(x - 1, y, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [xf, yf, zf], [xf, yf + 1.0, zf], [xf, yf + 1.0, zf + 1.0], [xf, yf, zf + 1.0], layer);
                }
                // +Z Front
                if z == CHUNK_SIZE - 1 || chunk.get_voxel(x, y, z + 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [xf, yf, zf + 1.0], [xf, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf + 1.0, yf, zf + 1.0], layer);
                }
                // -Z Back
                if z == 0 || chunk.get_voxel(x, y, z - 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                              [xf + 1.0, yf, zf], [xf + 1.0, yf + 1.0, zf], [xf, yf + 1.0, zf], [xf, yf, zf], layer);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             p0: [f32; 3], p1: [f32; 3], p2: [f32; 3], p3: [f32; 3], layer: u32) {
    verts.push(Vertex { pos: p0, uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: p1, uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: p2, uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: p3, uv: [1.0, 1.0], layer });
    // Winding order CCW
    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count, *count + 2, *count + 3]);
    *count += 4;
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_neighbor_culling() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        // Colocamos dos bloques adyacentes
        chunk.set_voxel(10, 10, 10, 1);
        chunk.set_voxel(11, 10, 10, 1);

        let mesh = generate_mesh(&chunk);

        // Cada cubo independiente tendría 6 caras * 4 vértices = 24 vértices (48 total)
        // Con culling, las caras compartidas (entre x=10 y x=11) no se dibujan.
        // 2 cubos adjuntos = 10 caras * 4 vértices = 40 vértices
        assert_eq!(mesh.vertices.len(), 40);
        // Cada cara tiene 2 triángulos * 3 índices = 6 índices. 10 caras = 60 índices.
        assert_eq!(mesh.indices.len(), 60);
    }
}