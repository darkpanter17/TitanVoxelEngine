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

#[allow(dead_code)]
fn is_air(chunk: &Chunk, x: i32, y: i32, z: i32) -> bool {
    if x < 0 || y < 0 || z < 0 || x >= CHUNK_SIZE as i32 || y >= CHUNK_HEIGHT as i32 || z >= CHUNK_SIZE as i32 {
        return true; // Consider out-of-bounds as air to draw edge faces
    }
    chunk.get_voxel(x as usize, y as usize, z as usize) == 0
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    // Greedy meshing for X faces
    for x in 0..=CHUNK_SIZE {
        let mut mask_left = [0u16; CHUNK_HEIGHT * CHUNK_SIZE];
        let mut mask_right = [0u16; CHUNK_HEIGHT * CHUNK_SIZE];

        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let voxel_0 = if x > 0 { chunk.get_voxel(x - 1, y, z) } else { 0 };
                let voxel_1 = if x < CHUNK_SIZE { chunk.get_voxel(x, y, z) } else { 0 };

                let idx = y * CHUNK_SIZE + z;
                mask_left[idx] = if voxel_0 != 0 && voxel_1 == 0 { voxel_0 } else { 0 };
                mask_right[idx] = if voxel_1 != 0 && voxel_0 == 0 { voxel_1 } else { 0 };
            }
        }

        greedy_mesh_2d(&mask_left, CHUNK_SIZE, CHUNK_HEIGHT, |y, z, w, h, id| {
            let xf = x as f32 + offset_x;
            let yf = y as f32 + offset_y;
            let zf = z as f32 + offset_z;
            let layer = id as u32 - 1;
            // Left face (-X)
            // w corresponds to z-axis (u), h corresponds to y-axis (v)
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf, yf, zf], [xf, yf, zf + w as f32], [xf, yf + h as f32, zf + w as f32], [xf, yf + h as f32, zf]],
                layer, w as f32, h as f32);
        });

        greedy_mesh_2d(&mask_right, CHUNK_SIZE, CHUNK_HEIGHT, |y, z, w, h, id| {
            let xf = x as f32 + offset_x;
            let yf = y as f32 + offset_y;
            let zf = z as f32 + offset_z;
            let layer = id as u32 - 1;
            // Right face (+X)
            // w corresponds to z-axis (u), h corresponds to y-axis (v)
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf, yf, zf + w as f32], [xf, yf, zf], [xf, yf + h as f32, zf], [xf, yf + h as f32, zf + w as f32]],
                layer, w as f32, h as f32);
        });
    }

    // Greedy meshing for Y faces
    for y in 0..=CHUNK_HEIGHT {
        let mut mask_bottom = [0u16; CHUNK_SIZE * CHUNK_SIZE];
        let mut mask_top = [0u16; CHUNK_SIZE * CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let voxel_0 = if y > 0 { chunk.get_voxel(x, y - 1, z) } else { 0 };
                let voxel_1 = if y < CHUNK_HEIGHT { chunk.get_voxel(x, y, z) } else { 0 };

                let idx = x * CHUNK_SIZE + z;
                // Note: mask index is x * CHUNK_SIZE + z (v = x, u = z)
                mask_bottom[idx] = if voxel_0 != 0 && voxel_1 == 0 { voxel_0 } else { 0 };
                mask_top[idx] = if voxel_1 != 0 && voxel_0 == 0 { voxel_1 } else { 0 };
            }
        }

        greedy_mesh_2d(&mask_bottom, CHUNK_SIZE, CHUNK_SIZE, |x, z, w, h, id| {
            let xf = x as f32 + offset_x;
            let yf = y as f32 + offset_y;
            let zf = z as f32 + offset_z;
            let layer = id as u32 - 1;
            // Bottom face (-Y)
            // mask index is v * dim_u + u, where v = x, u = z. So h corresponds to x-axis, w corresponds to z-axis.
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf, yf, zf], [xf + h as f32, yf, zf], [xf + h as f32, yf, zf + w as f32], [xf, yf, zf + w as f32]],
                layer, h as f32, w as f32);
        });

        greedy_mesh_2d(&mask_top, CHUNK_SIZE, CHUNK_SIZE, |x, z, w, h, id| {
            let xf = x as f32 + offset_x;
            let yf = y as f32 + offset_y;
            let zf = z as f32 + offset_z;
            let layer = id as u32 - 1;
            // Top face (+Y)
            // h corresponds to x-axis, w corresponds to z-axis
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf, yf, zf + w as f32], [xf + h as f32, yf, zf + w as f32], [xf + h as f32, yf, zf], [xf, yf, zf]],
                layer, h as f32, w as f32);
        });
    }

    // Greedy meshing for Z faces
    for z in 0..=CHUNK_SIZE {
        let mut mask_back = [0u16; CHUNK_SIZE * CHUNK_HEIGHT];
        let mut mask_front = [0u16; CHUNK_SIZE * CHUNK_HEIGHT];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                let voxel_0 = if z > 0 { chunk.get_voxel(x, y, z - 1) } else { 0 };
                let voxel_1 = if z < CHUNK_SIZE { chunk.get_voxel(x, y, z) } else { 0 };

                let idx = x * CHUNK_HEIGHT + y;
                // mask index is x * CHUNK_HEIGHT + y (v = x, u = y)
                mask_back[idx] = if voxel_0 != 0 && voxel_1 == 0 { voxel_0 } else { 0 };
                mask_front[idx] = if voxel_1 != 0 && voxel_0 == 0 { voxel_1 } else { 0 };
            }
        }

        greedy_mesh_2d(&mask_back, CHUNK_HEIGHT, CHUNK_SIZE, |x, y, w, h, id| {
            let xf = x as f32 + offset_x;
            let yf = y as f32 + offset_y;
            let zf = z as f32 + offset_z;
            let layer = id as u32 - 1;
            // Back face (-Z)
            // v = x, u = y, so h corresponds to x-axis, w corresponds to y-axis
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf + h as f32, yf, zf], [xf, yf, zf], [xf, yf + w as f32, zf], [xf + h as f32, yf + w as f32, zf]],
                layer, h as f32, w as f32);
        });

        greedy_mesh_2d(&mask_front, CHUNK_HEIGHT, CHUNK_SIZE, |x, y, w, h, id| {
            let xf = x as f32 + offset_x;
            let yf = y as f32 + offset_y;
            let zf = z as f32 + offset_z;
            let layer = id as u32 - 1;
            // Front face (+Z)
            // h corresponds to x-axis, w corresponds to y-axis
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf, yf, zf], [xf + h as f32, yf, zf], [xf + h as f32, yf + w as f32, zf], [xf, yf + w as f32, zf]],
                layer, h as f32, w as f32);
        });
    }

    Mesh { vertices, indices }
}

fn greedy_mesh_2d<F>(mask: &[u16], dim_u: usize, dim_v: usize, mut push_quad: F)
where
    F: FnMut(usize, usize, usize, usize, u16),
{
    let mut visited = vec![false; dim_u * dim_v];

    for v in 0..dim_v {
        for u in 0..dim_u {
            let idx = v * dim_u + u;
            let id = mask[idx];

            if id == 0 || visited[idx] {
                continue;
            }

            let mut w = 1;
            while u + w < dim_u && mask[v * dim_u + (u + w)] == id && !visited[v * dim_u + (u + w)] {
                w += 1;
            }

            let mut h = 1;
            'outer: while v + h < dim_v {
                for du in 0..w {
                    if mask[(v + h) * dim_u + (u + du)] != id || visited[(v + h) * dim_u + (u + du)] {
                        break 'outer;
                    }
                }
                h += 1;
            }

            for dv in 0..h {
                for du in 0..w {
                    visited[(v + dv) * dim_u + (u + du)] = true;
                }
            }

            push_quad(v, u, w, h, id);
        }
    }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             pos: [[f32; 3]; 4], layer: u32, width: f32, height: f32) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, height], layer });
    verts.push(Vertex { pos: pos[1], uv: [width, height], layer });
    verts.push(Vertex { pos: pos[2], uv: [width, 0.0], layer });
    verts.push(Vertex { pos: pos[3], uv: [0.0, 0.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_is_air() {
        let chunk = Chunk::new(IVec3::ZERO);
        assert!(is_air(&chunk, -1, 0, 0));
        assert!(is_air(&chunk, 0, -1, 0));
        assert!(is_air(&chunk, 0, 0, -1));
        assert!(is_air(&chunk, CHUNK_SIZE as i32, 0, 0));
        assert!(is_air(&chunk, 0, CHUNK_HEIGHT as i32, 0));
        assert!(is_air(&chunk, 0, 0, CHUNK_SIZE as i32));
    }
}