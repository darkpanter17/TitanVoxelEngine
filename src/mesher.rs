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
                let voxel_id = chunk.get_voxel(x, y, z);
                if voxel_id == 0 { continue; }

                // Map BlockID to Texture Layer (1->0, 2->1, etc.)
                let layer = (voxel_id - 1) as u32;

                let xf = x as f32;
                let yf = y as f32;
                let zf = z as f32;

                // Right (X+)
                if x + 1 >= CHUNK_SIZE || chunk.get_voxel(x + 1, y, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [xf + 1.0, yf, zf], [xf + 1.0, yf + 1.0, zf],
                        [xf + 1.0, yf + 1.0, zf + 1.0], [xf + 1.0, yf, zf + 1.0],
                        layer);
                }
                // Left (X-)
                if x == 0 || chunk.get_voxel(x - 1, y, z) == 0 {
                     push_face(&mut vertices, &mut indices, &mut index_count,
                        [xf, yf, zf + 1.0], [xf, yf + 1.0, zf + 1.0],
                        [xf, yf + 1.0, zf], [xf, yf, zf],
                        layer);
                }
                // Top (Y+)
                if y + 1 >= CHUNK_HEIGHT || chunk.get_voxel(x, y + 1, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [xf, yf + 1.0, zf], [xf, yf + 1.0, zf + 1.0],
                        [xf + 1.0, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf],
                        layer);
                }
                // Bottom (Y-)
                if y == 0 || chunk.get_voxel(x, y - 1, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [xf, yf, zf + 1.0], [xf, yf, zf],
                        [xf + 1.0, yf, zf], [xf + 1.0, yf, zf + 1.0],
                        layer);
                }
                // Back (Z+)
                if z + 1 >= CHUNK_SIZE || chunk.get_voxel(x, y, z + 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0],
                        [xf, yf + 1.0, zf + 1.0], [xf, yf, zf + 1.0],
                        layer);
                }
                // Front (Z-)
                if z == 0 || chunk.get_voxel(x, y, z - 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [xf, yf, zf], [xf, yf + 1.0, zf],
                        [xf + 1.0, yf + 1.0, zf], [xf + 1.0, yf, zf],
                        layer);
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             p1: [f32;3], p2: [f32;3], p3: [f32;3], p4: [f32;3], layer: u32) {
    // p1 bottom-left, p2 top-left, p3 top-right, p4 bottom-right
    // UVs assume standard top-left origin for texture sampling
    verts.push(Vertex { pos: p1, uv: [0.0, 1.0], layer }); // BL
    verts.push(Vertex { pos: p2, uv: [0.0, 0.0], layer }); // TL
    verts.push(Vertex { pos: p3, uv: [1.0, 0.0], layer }); // TR
    verts.push(Vertex { pos: p4, uv: [1.0, 1.0], layer }); // BR

    // Triangle 1: BL -> TR -> TL (CCW)
    inds.push(*count);
    inds.push(*count + 2);
    inds.push(*count + 1);
    // Triangle 2: BL -> BR -> TR (CCW)
    inds.push(*count);
    inds.push(*count + 3);
    inds.push(*count + 2);

    *count += 4;
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_single_voxel_mesh() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(0, 0, 0, 1);
        let mesh = generate_mesh(&chunk);

        // Single isolated voxel should have 6 faces.
        // Each face = 4 vertices. Total 24 vertices.
        // Each face = 2 triangles = 6 indices. Total 36 indices.
        assert_eq!(mesh.vertices.len(), 24);
        assert_eq!(mesh.indices.len(), 36);
    }

    #[test]
    fn test_two_adjacent_voxels() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(0, 0, 0, 1);
        chunk.set_voxel(1, 0, 0, 1);
        let mesh = generate_mesh(&chunk);

        // Two adjacent voxels along X.
        // Voxel 1 (0,0,0): Right face hidden. 5 faces.
        // Voxel 2 (1,0,0): Left face hidden. 5 faces.
        // Total 10 faces.
        // 10 * 4 = 40 vertices.
        // 10 * 6 = 60 indices.
        assert_eq!(mesh.vertices.len(), 40);
        assert_eq!(mesh.indices.len(), 60);
    }
}
