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

fn is_air(chunk: &Chunk, x: i32, y: i32, z: i32) -> bool {
    if x < 0 || y < 0 || z < 0 || x >= CHUNK_SIZE as i32 || y >= CHUNK_HEIGHT as i32 || z >= CHUNK_SIZE as i32 {
        return true; // Consider out-of-bounds as air to draw edge faces
    }
    chunk.get_voxel(x as usize, y as usize, z as usize) == 0
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

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    // Y-axis (Top and Bottom)
    for top in [true, false] {
        let dir = if top { 1 } else { -1 };
        for y in 0..CHUNK_HEIGHT {
            let mut mask = vec![0u16; CHUNK_SIZE * CHUNK_SIZE];
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let id = chunk.get_voxel(x, y, z);
                    if id != 0 && is_air(chunk, x as i32, y as i32 + dir, z as i32) {
                        mask[x + z * CHUNK_SIZE] = id;
                    }
                }
            }

            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let id = mask[x + z * CHUNK_SIZE];
                    if id != 0 {
                        let mut w = 1;
                        while x + w < CHUNK_SIZE && mask[x + w + z * CHUNK_SIZE] == id {
                            w += 1;
                        }
                        let mut h = 1;
                        'outer: while z + h < CHUNK_SIZE {
                            for k in 0..w {
                                if mask[x + k + (z + h) * CHUNK_SIZE] != id {
                                    break 'outer;
                                }
                            }
                            h += 1;
                        }

                        for dz in 0..h {
                            for dx in 0..w {
                                mask[x + dx + (z + dz) * CHUNK_SIZE] = 0;
                            }
                        }

                        let layer = id as u32 - 1;
                        let xf = x as f32 + offset_x;
                        let yf = y as f32 + offset_y + if top { 1.0 } else { 0.0 };
                        let zf = z as f32 + offset_z;
                        let wf = w as f32;
                        let hf = h as f32;

                        if top {
                            push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf, yf, zf + hf], [xf + wf, yf, zf + hf], [xf + wf, yf, zf], [xf, yf, zf]], layer, wf, hf);
                        } else {
                            push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf, yf, zf], [xf + wf, yf, zf], [xf + wf, yf, zf + hf], [xf, yf, zf + hf]], layer, wf, hf);
                        }
                    }
                }
            }
        }
    }

    // X-axis (Right and Left)
    for right in [true, false] {
        let dir = if right { 1 } else { -1 };
        for x in 0..CHUNK_SIZE {
            let mut mask = vec![0u16; CHUNK_SIZE * CHUNK_HEIGHT];
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let id = chunk.get_voxel(x, y, z);
                    if id != 0 && is_air(chunk, x as i32 + dir, y as i32, z as i32) {
                        mask[z + y * CHUNK_SIZE] = id;
                    }
                }
            }

            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let id = mask[z + y * CHUNK_SIZE];
                    if id != 0 {
                        let mut w = 1;
                        while z + w < CHUNK_SIZE && mask[z + w + y * CHUNK_SIZE] == id {
                            w += 1;
                        }
                        let mut h = 1;
                        'outer: while y + h < CHUNK_HEIGHT {
                            for k in 0..w {
                                if mask[z + k + (y + h) * CHUNK_SIZE] != id {
                                    break 'outer;
                                }
                            }
                            h += 1;
                        }

                        for dy in 0..h {
                            for dz in 0..w {
                                mask[z + dz + (y + dy) * CHUNK_SIZE] = 0;
                            }
                        }

                        let layer = id as u32 - 1;
                        let xf = x as f32 + offset_x + if right { 1.0 } else { 0.0 };
                        let yf = y as f32 + offset_y;
                        let zf = z as f32 + offset_z;
                        let wf = w as f32;
                        let hf = h as f32;

                        if right { // +X
                            push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf, yf, zf + wf], [xf, yf + hf, zf + wf], [xf, yf + hf, zf], [xf, yf, zf]], layer, wf, hf);
                        } else { // -X
                            push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf, yf, zf], [xf, yf + hf, zf], [xf, yf + hf, zf + wf], [xf, yf, zf + wf]], layer, wf, hf);
                        }
                    }
                }
            }
        }
    }

    // Z-axis (Front and Back)
    for front in [true, false] {
        let dir = if front { 1 } else { -1 };
        for z in 0..CHUNK_SIZE {
            let mut mask = vec![0u16; CHUNK_SIZE * CHUNK_HEIGHT];
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_SIZE {
                    let id = chunk.get_voxel(x, y, z);
                    if id != 0 && is_air(chunk, x as i32, y as i32, z as i32 + dir) {
                        mask[x + y * CHUNK_SIZE] = id;
                    }
                }
            }

            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_SIZE {
                    let id = mask[x + y * CHUNK_SIZE];
                    if id != 0 {
                        let mut w = 1;
                        while x + w < CHUNK_SIZE && mask[x + w + y * CHUNK_SIZE] == id {
                            w += 1;
                        }
                        let mut h = 1;
                        'outer: while y + h < CHUNK_HEIGHT {
                            for k in 0..w {
                                if mask[x + k + (y + h) * CHUNK_SIZE] != id {
                                    break 'outer;
                                }
                            }
                            h += 1;
                        }

                        for dy in 0..h {
                            for dx in 0..w {
                                mask[x + dx + (y + dy) * CHUNK_SIZE] = 0;
                            }
                        }

                        let layer = id as u32 - 1;
                        let xf = x as f32 + offset_x;
                        let yf = y as f32 + offset_y;
                        let zf = z as f32 + offset_z + if front { 1.0 } else { 0.0 };
                        let wf = w as f32;
                        let hf = h as f32;

                        if front { // +Z
                            push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf, yf, zf], [xf, yf + hf, zf], [xf + wf, yf + hf, zf], [xf + wf, yf, zf]], layer, wf, hf);
                        } else { // -Z
                            push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf + wf, yf, zf], [xf + wf, yf + hf, zf], [xf, yf + hf, zf], [xf, yf, zf]], layer, wf, hf);
                        }
                    }
                }
            }
        }
    }

    Mesh { vertices, indices }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_is_air_bounds() {
        let chunk = Chunk::new(IVec3::ZERO);
        assert!(is_air(&chunk, -1, 0, 0));
        assert!(is_air(&chunk, CHUNK_SIZE as i32, 0, 0));
        assert!(is_air(&chunk, 0, -1, 0));
        assert!(is_air(&chunk, 0, CHUNK_HEIGHT as i32, 0));
        assert!(is_air(&chunk, 0, 0, -1));
        assert!(is_air(&chunk, 0, 0, CHUNK_SIZE as i32));
    }

    #[test]
    fn test_push_face_winding_and_uv() {
        let mut verts = Vec::new();
        let mut inds = Vec::new();
        let mut count = 0;

        let width = 2.0;
        let height = 3.0;
        let pos = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0]
        ];

        push_face(&mut verts, &mut inds, &mut count, pos, 0, width, height);

        assert_eq!(verts.len(), 4);
        assert_eq!(inds.len(), 6);
        assert_eq!(count, 4);

        assert_eq!(verts[0].uv, [0.0, height]);
        assert_eq!(verts[1].uv, [width, height]);
        assert_eq!(verts[2].uv, [width, 0.0]);
        assert_eq!(verts[3].uv, [0.0, 0.0]);

        assert_eq!(inds, vec![0, 1, 2, 2, 3, 0]);
    }
}
