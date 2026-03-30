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
fn get_voxel_safe(chunk: &Chunk, x: i32, y: i32, z: i32) -> u16 {
    if x < 0 || x >= CHUNK_SIZE as i32 ||
       y < 0 || y >= CHUNK_HEIGHT as i32 ||
       z < 0 || z >= CHUNK_SIZE as i32 {
        return 0; // Fuera del chunk se considera aire por ahora (en un motor real revisaríamos el chunk vecino)
    }
    chunk.get_voxel(x as usize, y as usize, z as usize)
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

                 let layer = voxel as u32 - 1; // ID 1 -> Layer 0

                 let ix = x as i32; let iy = y as i32; let iz = z as i32;
                 let px = x as f32 + offset_x;
                 let py = y as f32 + offset_y;
                 let pz = z as f32 + offset_z;

                 // Back face (-Z)
                 if get_voxel_safe(chunk, ix, iy, iz - 1) == 0 {
                     push_quad(&mut vertices, &mut indices, &mut index_count,
                               [[px, py, pz], [px, py + 1.0, pz], [px + 1.0, py + 1.0, pz], [px + 1.0, py, pz]], layer);
                 }
                 // Front face (+Z)
                 if get_voxel_safe(chunk, ix, iy, iz + 1) == 0 {
                     push_quad(&mut vertices, &mut indices, &mut index_count,
                               [[px + 1.0, py, pz + 1.0], [px + 1.0, py + 1.0, pz + 1.0], [px, py + 1.0, pz + 1.0], [px, py, pz + 1.0]], layer);
                 }
                 // Left face (-X)
                 if get_voxel_safe(chunk, ix - 1, iy, iz) == 0 {
                     push_quad(&mut vertices, &mut indices, &mut index_count,
                               [[px, py, pz + 1.0], [px, py + 1.0, pz + 1.0], [px, py + 1.0, pz], [px, py, pz]], layer);
                 }
                 // Right face (+X)
                 if get_voxel_safe(chunk, ix + 1, iy, iz) == 0 {
                     push_quad(&mut vertices, &mut indices, &mut index_count,
                               [[px + 1.0, py, pz], [px + 1.0, py + 1.0, pz], [px + 1.0, py + 1.0, pz + 1.0], [px + 1.0, py, pz + 1.0]], layer);
                 }
                 // Bottom face (-Y)
                 if get_voxel_safe(chunk, ix, iy - 1, iz) == 0 {
                     push_quad(&mut vertices, &mut indices, &mut index_count,
                               [[px, py, pz + 1.0], [px, py, pz], [px + 1.0, py, pz], [px + 1.0, py, pz + 1.0]], layer);
                 }
                 // Top face (+Y)
                 if get_voxel_safe(chunk, ix, iy + 1, iz) == 0 {
                     push_quad(&mut vertices, &mut indices, &mut index_count,
                               [[px, py + 1.0, pz], [px, py + 1.0, pz + 1.0], [px + 1.0, py + 1.0, pz + 1.0], [px + 1.0, py + 1.0, pz]], layer);
                 }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_quad(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, 
             positions: [[f32; 3]; 4], layer: u32) {
    verts.push(Vertex { pos: positions[0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: positions[1], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: positions[2], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: positions[3], uv: [1.0, 1.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
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

        // Un solo voxel rodeado de aire debería tener 6 caras = 24 vértices, 36 índices
        assert_eq!(mesh.vertices.len(), 24);
        assert_eq!(mesh.indices.len(), 36);
    }

    #[test]
    fn test_two_adjacent_voxels() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(0, 0, 0, 1);
        chunk.set_voxel(1, 0, 0, 1);

        let mesh = generate_mesh(&chunk);

        // Dos voxeles adyacentes deberían compartir una cara (que es eliminada)
        // Por lo tanto, tendrían 10 caras expuestas = 40 vértices, 60 índices
        assert_eq!(mesh.vertices.len(), 40);
        assert_eq!(mesh.indices.len(), 60);
    }
}