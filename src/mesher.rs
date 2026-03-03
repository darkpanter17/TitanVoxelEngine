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

// Helper functions para chequear bloques vecinos con culling intra-chunk.
// En un motor completo se comprobaría contra chunks vecinos usando World.
fn is_transparent(chunk: &Chunk, x: i32, y: i32, z: i32) -> bool {
    if x < 0 || y < 0 || z < 0 || x >= CHUNK_SIZE as i32 || y >= CHUNK_HEIGHT as i32 || z >= CHUNK_SIZE as i32 {
        return true; // Asumir aire fuera del chunk para culling básico
    }
    chunk.get_voxel(x as usize, y as usize, z as usize) == 0
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let base_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let base_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let base_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                 let id = chunk.get_voxel(x, y, z);
                 if id != 0 {
                    let layer = id as u32;
                    let wx = base_x + x as f32;
                    let wy = base_y + y as f32;
                    let wz = base_z + z as f32;
                    let ix = x as i32; let iy = y as i32; let iz = z as i32;

                    // Top (Y+)
                    if is_transparent(chunk, ix, iy + 1, iz) {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [wx, wy+1.0, wz], [wx, wy+1.0, wz+1.0], [wx+1.0, wy+1.0, wz+1.0], [wx+1.0, wy+1.0, wz], layer);
                    }
                    // Bottom (Y-)
                    if is_transparent(chunk, ix, iy - 1, iz) {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [wx, wy, wz+1.0], [wx, wy, wz], [wx+1.0, wy, wz], [wx+1.0, wy, wz+1.0], layer);
                    }
                    // Front (Z+)
                    if is_transparent(chunk, ix, iy, iz + 1) {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [wx+1.0, wy, wz+1.0], [wx+1.0, wy+1.0, wz+1.0], [wx, wy+1.0, wz+1.0], [wx, wy, wz+1.0], layer);
                    }
                    // Back (Z-)
                    if is_transparent(chunk, ix, iy, iz - 1) {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [wx, wy, wz], [wx, wy+1.0, wz], [wx+1.0, wy+1.0, wz], [wx+1.0, wy, wz], layer);
                    }
                    // Right (X+)
                    if is_transparent(chunk, ix + 1, iy, iz) {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [wx+1.0, wy, wz], [wx+1.0, wy+1.0, wz], [wx+1.0, wy+1.0, wz+1.0], [wx+1.0, wy, wz+1.0], layer);
                    }
                    // Left (X-)
                    if is_transparent(chunk, ix - 1, iy, iz) {
                        push_face(&mut vertices, &mut indices, &mut index_count,
                            [wx, wy, wz+1.0], [wx, wy+1.0, wz+1.0], [wx, wy+1.0, wz], [wx, wy, wz], layer);
                    }
                 }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             v0: [f32; 3], v1: [f32; 3], v2: [f32; 3], v3: [f32; 3], layer: u32) {
    verts.push(Vertex { pos: v0, uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: v1, uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: v2, uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: v3, uv: [1.0, 1.0], layer });
    // CCW (Counter-Clockwise)
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_generate_mesh_empty_chunk() {
        let chunk = Chunk::new(IVec3::ZERO);
        let mesh = generate_mesh(&chunk);
        assert!(mesh.vertices.is_empty());
        assert!(mesh.indices.is_empty());
    }

    #[test]
    fn test_generate_mesh_with_voxels() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        // Put a voxel at (0, 0, 0)
        chunk.set_voxel(0, 0, 0, 1);

        let mesh = generate_mesh(&chunk);

        // Should generate 6 quads (1 for each face) = 24 vertices, 36 indices
        assert_eq!(mesh.vertices.len(), 24);
        assert_eq!(mesh.indices.len(), 36);

        // Verify vertex properties
        assert_eq!(mesh.vertices[0].layer, 1);
    }
}