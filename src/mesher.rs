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

// Enumeración para las caras del voxel para facilitar el culling
#[derive(Copy, Clone)]
enum Face {
    Top, Bottom, Front, Back, Left, Right
}

// (Mantenemos la lógica de Greedy Meshing igual, solo cambia el Vertex struct arriba)
pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..100 { // Dibujamos hasta altura 100
                 let voxel_id = chunk.get_voxel(x, y, z);
                 if voxel_id != 0 {
                    let xf = x as f32; let yf = y as f32; let zf = z as f32;
                    let layer = (voxel_id - 1) as u32;

                    // Cara Superior (Top / Y+)
                    if y == 99 || chunk.get_voxel(x, y + 1, z) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count, xf, yf, zf, layer, Face::Top);
                    }
                    // Cara Inferior (Bottom / Y-)
                    if y == 0 || chunk.get_voxel(x, y - 1, z) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count, xf, yf, zf, layer, Face::Bottom);
                    }
                    // Cara Frontal (Front / Z+)
                    if z == CHUNK_SIZE - 1 || chunk.get_voxel(x, y, z + 1) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count, xf, yf, zf, layer, Face::Front);
                    }
                    // Cara Trasera (Back / Z-)
                    if z == 0 || chunk.get_voxel(x, y, z - 1) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count, xf, yf, zf, layer, Face::Back);
                    }
                    // Cara Derecha (Right / X+)
                    if x == CHUNK_SIZE - 1 || chunk.get_voxel(x + 1, y, z) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count, xf, yf, zf, layer, Face::Right);
                    }
                    // Cara Izquierda (Left / X-)
                    if x == 0 || chunk.get_voxel(x - 1, y, z) == 0 {
                        push_face(&mut vertices, &mut indices, &mut index_count, xf, yf, zf, layer, Face::Left);
                    }
                 }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             x: f32, y: f32, z: f32, layer: u32, face: Face) {
    let (v0, v1, v2, v3) = match face {
        Face::Top => (
            [x, y + 1.0, z],
            [x, y + 1.0, z + 1.0],
            [x + 1.0, y + 1.0, z + 1.0],
            [x + 1.0, y + 1.0, z],
        ),
        Face::Bottom => (
            [x, y, z],
            [x + 1.0, y, z],
            [x + 1.0, y, z + 1.0],
            [x, y, z + 1.0],
        ),
        Face::Front => (
            [x, y, z + 1.0],
            [x + 1.0, y, z + 1.0],
            [x + 1.0, y + 1.0, z + 1.0],
            [x, y + 1.0, z + 1.0],
        ),
        Face::Back => (
            [x + 1.0, y, z],
            [x, y, z],
            [x, y + 1.0, z],
            [x + 1.0, y + 1.0, z],
        ),
        Face::Right => (
            [x + 1.0, y, z + 1.0],
            [x + 1.0, y, z],
            [x + 1.0, y + 1.0, z],
            [x + 1.0, y + 1.0, z + 1.0],
        ),
        Face::Left => (
            [x, y, z],
            [x, y, z + 1.0],
            [x, y + 1.0, z + 1.0],
            [x, y + 1.0, z],
        ),
    };

    verts.push(Vertex { pos: v0, uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: v1, uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: v2, uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: v3, uv: [0.0, 0.0], layer });

    // Winding order CCW for Back Face culling mode.
    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count + 2, *count + 3, *count]);
    *count += 4;
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_empty_chunk_mesh() {
        let chunk = Chunk::new(IVec3::ZERO);
        let mesh = generate_mesh(&chunk);
        assert_eq!(mesh.vertices.len(), 0);
        assert_eq!(mesh.indices.len(), 0);
    }

    #[test]
    fn test_single_voxel_mesh() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(15, 15, 15, 1);
        let mesh = generate_mesh(&chunk);

        // 1 block = 6 faces, 4 vertices per face = 24 vertices
        assert_eq!(mesh.vertices.len(), 24);

        // 6 faces, 6 indices per face = 36 indices
        assert_eq!(mesh.indices.len(), 36);
    }

    #[test]
    fn test_surrounded_voxel_mesh() {
        let mut chunk = Chunk::new(IVec3::ZERO);

        // Block we test
        chunk.set_voxel(15, 15, 15, 1);

        // Surround it with opaque blocks
        chunk.set_voxel(15, 16, 15, 1); // Top
        chunk.set_voxel(15, 14, 15, 1); // Bottom
        chunk.set_voxel(15, 15, 16, 1); // Front
        chunk.set_voxel(15, 15, 14, 1); // Back
        chunk.set_voxel(16, 15, 15, 1); // Right
        chunk.set_voxel(14, 15, 15, 1); // Left

        let mesh = generate_mesh(&chunk);

        // The center block should have 0 faces exposed
        // Each of the 6 surrounding blocks has exactly 5 faces exposed
        // 6 blocks * 5 faces/block * 4 vertices/face = 120 vertices
        // 6 blocks * 5 faces/block * 6 indices/face = 180 indices
        assert_eq!(mesh.vertices.len(), 120);
        assert_eq!(mesh.indices.len(), 180);
    }
}