use crate::chunk::{Chunk, CHUNK_HEIGHT, CHUNK_SIZE};
use crate::world::World;
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

// (Mantenemos la lógica de Greedy Meshing igual, solo cambia el Vertex struct arriba)
pub fn generate_mesh(chunk: &Chunk, world: &World) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;

    let chunk_world_x = chunk.position.x * CHUNK_SIZE as i32;
    let chunk_world_z = chunk.position.z * CHUNK_SIZE as i32;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                let voxel_id = chunk.get_voxel(x, y, z);
                if voxel_id == 0 {
                    continue;
                }

                let global_x = chunk_world_x + x as i32;
                let global_y = y as i32;
                let global_z = chunk_world_z + z as i32;

                let xf = global_x as f32;
                let yf = global_y as f32;
                let zf = global_z as f32;

                let layer = voxel_id as u32 - 1;

                // Check Top (Y+)
                if world.get_voxel(global_x, global_y + 1, global_z) == 0 {
                    push_face(
                        &mut vertices,
                        &mut indices,
                        &mut index_count,
                        [
                            [xf, yf + 1.0, zf],
                            [xf, yf + 1.0, zf + 1.0],
                            [xf + 1.0, yf + 1.0, zf + 1.0],
                            [xf + 1.0, yf + 1.0, zf],
                        ],
                        layer,
                    );
                }

                // Check Bottom (Y-)
                if global_y > 0 && world.get_voxel(global_x, global_y - 1, global_z) == 0 {
                    push_face(
                        &mut vertices,
                        &mut indices,
                        &mut index_count,
                        [
                            [xf, yf, zf + 1.0],
                            [xf, yf, zf],
                            [xf + 1.0, yf, zf],
                            [xf + 1.0, yf, zf + 1.0],
                        ],
                        layer,
                    );
                }

                // Check Left (X-)
                if world.get_voxel(global_x - 1, global_y, global_z) == 0 {
                    push_face(
                        &mut vertices,
                        &mut indices,
                        &mut index_count,
                        [
                            [xf, yf, zf],
                            [xf, yf, zf + 1.0],
                            [xf, yf + 1.0, zf + 1.0],
                            [xf, yf + 1.0, zf],
                        ],
                        layer,
                    );
                }

                // Check Right (X+)
                if world.get_voxel(global_x + 1, global_y, global_z) == 0 {
                    push_face(
                        &mut vertices,
                        &mut indices,
                        &mut index_count,
                        [
                            [xf + 1.0, yf, zf + 1.0],
                            [xf + 1.0, yf, zf],
                            [xf + 1.0, yf + 1.0, zf],
                            [xf + 1.0, yf + 1.0, zf + 1.0],
                        ],
                        layer,
                    );
                }

                // Check Front (Z+)
                if world.get_voxel(global_x, global_y, global_z + 1) == 0 {
                    push_face(
                        &mut vertices,
                        &mut indices,
                        &mut index_count,
                        [
                            [xf + 1.0, yf, zf + 1.0],
                            [xf + 1.0, yf + 1.0, zf + 1.0],
                            [xf, yf + 1.0, zf + 1.0],
                            [xf, yf, zf + 1.0],
                        ],
                        layer,
                    );
                }

                // Check Back (Z-)
                if world.get_voxel(global_x, global_y, global_z - 1) == 0 {
                    push_face(
                        &mut vertices,
                        &mut indices,
                        &mut index_count,
                        [
                            [xf, yf, zf],
                            [xf, yf + 1.0, zf],
                            [xf + 1.0, yf + 1.0, zf],
                            [xf + 1.0, yf, zf],
                        ],
                        layer,
                    );
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(
    verts: &mut Vec<Vertex>,
    inds: &mut Vec<u32>,
    count: &mut u32,
    positions: [[f32; 3]; 4],
    layer: u32,
) {
    verts.push(Vertex {
        pos: positions[0],
        uv: [0.0, 1.0],
        layer,
    });
    verts.push(Vertex {
        pos: positions[1],
        uv: [0.0, 0.0],
        layer,
    });
    verts.push(Vertex {
        pos: positions[2],
        uv: [1.0, 0.0],
        layer,
    });
    verts.push(Vertex {
        pos: positions[3],
        uv: [1.0, 1.0],
        layer,
    });
    // Counter-clockwise
    inds.extend_from_slice(&[
        *count,
        *count + 1,
        *count + 2,
        *count + 2,
        *count + 3,
        *count,
    ]);
    *count += 4;
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_meshing_single_voxel() {
        let mut world = World::new();
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(0, 0, 0, 1);
        world.insert_chunk(chunk);

        let chunk_ref = world.get_chunk(&IVec3::ZERO).unwrap();
        let mesh = generate_mesh(chunk_ref, &world);

        // Check Top (Y+), Check Bottom (Y-) is false (because y=0 global_y>0 check fails so bottom face isn't drawn). Wait, Y- isn't drawn because y=0!
        // The check for Bottom (Y-) is `if global_y > 0 && ...` so at global_y=0, the bottom face is NOT drawn.
        // So a single voxel at y=0 has 5 faces drawn (Top, Left, Right, Front, Back).
        // 5 faces * 4 vertices = 20 vertices
        // 5 faces * 6 indices = 30 indices
        assert_eq!(mesh.vertices.len(), 20);
        assert_eq!(mesh.indices.len(), 30);
    }

    #[test]
    fn test_meshing_two_voxels_culling() {
        let mut world = World::new();
        let mut chunk = Chunk::new(IVec3::ZERO);
        // Place two voxels adjacent to each other
        chunk.set_voxel(0, 0, 0, 1);
        chunk.set_voxel(1, 0, 0, 1);
        world.insert_chunk(chunk);

        let chunk_ref = world.get_chunk(&IVec3::ZERO).unwrap();
        let mesh = generate_mesh(chunk_ref, &world);

        // Normally 2 voxels = 12 faces.
        // Bottom faces (y=0) are not drawn (2 faces).
        // Adjacent faces are culled (2 faces).
        // 12 - 2 (bottom) - 2 (adjacent) = 8 faces.
        // 8 faces * 4 vertices = 32 vertices
        // 8 faces * 6 indices = 48 indices
        assert_eq!(mesh.vertices.len(), 32);
        assert_eq!(mesh.indices.len(), 48);
    }
}
