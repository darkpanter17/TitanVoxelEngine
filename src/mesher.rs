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

    for axis in 0..3 {
        let (u, v) = match axis {
            0 => (1, 2), // X -> Y, Z
            1 => (0, 2), // Y -> X, Z
            _ => (0, 1), // Z -> X, Y
        };

        let u_size = if u == 1 { CHUNK_HEIGHT } else { CHUNK_SIZE };
        let v_size = if v == 1 { CHUNK_HEIGHT } else { CHUNK_SIZE };
        let w_size = if axis == 1 { CHUNK_HEIGHT } else { CHUNK_SIZE };

        for w in -1..(w_size as i32) {
            for back_face in [false, true].iter() {
                let mut mask = vec![0u16; u_size * v_size];

                for v_idx in 0..v_size {
                    for u_idx in 0..u_size {
                        let mut coords1 = [0i32; 3];
                        coords1[u] = u_idx as i32;
                        coords1[v] = v_idx as i32;
                        coords1[axis] = w + if *back_face { 0 } else { 1 };

                        let mut coords2 = [0i32; 3];
                        coords2[u] = u_idx as i32;
                        coords2[v] = v_idx as i32;
                        coords2[axis] = w + if *back_face { 1 } else { 0 };

                        let b1_valid = !(coords1[0] < 0 || coords1[1] < 0 || coords1[2] < 0 || coords1[0] >= CHUNK_SIZE as i32 || coords1[1] >= CHUNK_HEIGHT as i32 || coords1[2] >= CHUNK_SIZE as i32);
                        let b2_valid = !(coords2[0] < 0 || coords2[1] < 0 || coords2[2] < 0 || coords2[0] >= CHUNK_SIZE as i32 || coords2[1] >= CHUNK_HEIGHT as i32 || coords2[2] >= CHUNK_SIZE as i32);

                        let id1 = if b1_valid { chunk.get_voxel(coords1[0] as usize, coords1[1] as usize, coords1[2] as usize) } else { 0 };
                        let id2 = if b2_valid { chunk.get_voxel(coords2[0] as usize, coords2[1] as usize, coords2[2] as usize) } else { 0 };

                        if id1 != 0 && id2 == 0 {
                            mask[u_idx + v_idx * u_size] = id1;
                        }
                    }
                }

                let mut j = 0;
                while j < v_size {
                    let mut i = 0;
                    while i < u_size {
                        let id = mask[i + j * u_size];
                        if id != 0 {
                            let mut width = 1;
                            while i + width < u_size && mask[i + width + j * u_size] == id {
                                width += 1;
                            }

                            let mut height = 1;
                            'outer: while j + height < v_size {
                                for k in 0..width {
                                    if mask[i + k + (j + height) * u_size] != id {
                                        break 'outer;
                                    }
                                }
                                height += 1;
                            }

                            let mut du = [0.0; 3];
                            let mut dv = [0.0; 3];
                            du[u] = width as f32;
                            dv[v] = height as f32;

                            let mut q = [0.0; 3];
                            q[u] = i as f32;
                            q[v] = j as f32;
                            q[axis] = (w + if *back_face { 0 } else { 1 }) as f32;

                            let bl = [q[0] + offset_x, q[1] + offset_y, q[2] + offset_z];
                            let br = [q[0] + du[0] + offset_x, q[1] + du[1] + offset_y, q[2] + du[2] + offset_z];
                            let tr = [q[0] + du[0] + dv[0] + offset_x, q[1] + du[1] + dv[1] + offset_y, q[2] + du[2] + dv[2] + offset_z];
                            let tl = [q[0] + dv[0] + offset_x, q[1] + dv[1] + offset_y, q[2] + dv[2] + offset_z];

                            let layer = (id - 1) as u32;

                            let mut pos = [bl, br, tr, tl];

                            if axis == 0 {
                                if *back_face {
                                    pos = [pos[0], pos[3], pos[2], pos[1]];
                                }
                            } else if axis == 1 {
                                if !*back_face {
                                    pos = [pos[0], pos[3], pos[2], pos[1]];
                                }
                            } else if axis == 2 {
                                if !*back_face {
                                    pos = [pos[0], pos[3], pos[2], pos[1]];
                                }
                            }

                            push_face(&mut vertices, &mut indices, &mut index_count, pos, layer, width as f32, height as f32);

                            for l in 0..height {
                                for k in 0..width {
                                    mask[i + k + (j + l) * u_size] = 0;
                                }
                            }
                            i += width;
                        } else {
                            i += 1;
                        }
                    }
                    j += 1;
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             pos: [[f32; 3]; 4], layer: u32, width: f32, height: f32) {
    // CCW mapping: [Bottom-Left, Bottom-Right, Top-Right, Top-Left]
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
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(0, 0, 0, 1);
        chunk.set_voxel(1, 1, 1, 0);

        // Inside bounds
        assert_eq!(is_air(&chunk, 0, 0, 0), false);
        assert_eq!(is_air(&chunk, 1, 1, 1), true);

        // Out of bounds
        assert_eq!(is_air(&chunk, -1, 0, 0), true);
        assert_eq!(is_air(&chunk, 0, -1, 0), true);
        assert_eq!(is_air(&chunk, 0, 0, -1), true);
        assert_eq!(is_air(&chunk, CHUNK_SIZE as i32, 0, 0), true);
        assert_eq!(is_air(&chunk, 0, CHUNK_HEIGHT as i32, 0), true);
        assert_eq!(is_air(&chunk, 0, 0, CHUNK_SIZE as i32), true);
    }
}