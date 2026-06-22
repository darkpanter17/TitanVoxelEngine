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

#[inline]
fn get_voxel(chunk: &Chunk, x: i32, y: i32, z: i32) -> u16 {
    if x < 0 || y < 0 || z < 0 || x >= CHUNK_SIZE as i32 || y >= CHUNK_HEIGHT as i32 || z >= CHUNK_SIZE as i32 {
        0
    } else {
        chunk.get_voxel(x as usize, y as usize, z as usize)
    }
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    // 6 directions: Top (+Y), Bottom (-Y), Right (+X), Left (-X), Front (+Z), Back (-Z)
    for dir in 0..6 {
        // Dimension mapping for greedy meshing
        // u, v are the indices we iterate over, d is the slice depth
        let (u_max, v_max, d_max) = match dir {
            0 | 1 => (CHUNK_SIZE, CHUNK_SIZE, CHUNK_HEIGHT), // Top/Bottom (XZ plane)
            2 | 3 => (CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SIZE), // Right/Left (ZY plane)
            4 | 5 => (CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SIZE), // Front/Back (XY plane)
            _ => unreachable!(),
        };

        for d in 0..d_max {
            let mut mask = vec![0u16; u_max * v_max];

            // Step 1: Generate mask
            for v in 0..v_max {
                for u in 0..u_max {
                    // Map (u, v, d) back to (x, y, z)
                    let (x, y, z) = match dir {
                        0 | 1 => (u as i32, d as i32, v as i32),
                        2 | 3 => (d as i32, v as i32, u as i32),
                        4 | 5 => (u as i32, v as i32, d as i32),
                        _ => unreachable!(),
                    };

                    let current = get_voxel(chunk, x, y, z);

                    if current != 0 {
                        // Check neighbor in direction
                        let neighbor = match dir {
                            0 => get_voxel(chunk, x, y + 1, z),
                            1 => get_voxel(chunk, x, y - 1, z),
                            2 => get_voxel(chunk, x + 1, y, z),
                            3 => get_voxel(chunk, x - 1, y, z),
                            4 => get_voxel(chunk, x, y, z + 1),
                            5 => get_voxel(chunk, x, y, z - 1),
                            _ => unreachable!(),
                        };

                        // If neighbor is air (or transparent), face is exposed
                        if neighbor == 0 {
                            mask[u + v * u_max] = current;
                        }
                    }
                }
            }

            // Step 2: Greedy meshing over mask
            for v in 0..v_max {
                let mut u = 0;
                while u < u_max {
                    let voxel_id = mask[u + v * u_max];
                    if voxel_id != 0 {
                        // Find width (w) in u
                        let mut w = 1;
                        while u + w < u_max && mask[(u + w) + v * u_max] == voxel_id {
                            w += 1;
                        }

                        // Find height (h) in v
                        let mut h = 1;
                        'height_loop: while v + h < v_max {
                            for w_idx in 0..w {
                                if mask[(u + w_idx) + (v + h) * u_max] != voxel_id {
                                    break 'height_loop;
                                }
                            }
                            h += 1;
                        }

                        // Clear the mask for this rectangle
                        for h_idx in 0..h {
                            for w_idx in 0..w {
                                mask[(u + w_idx) + (v + h_idx) * u_max] = 0;
                            }
                        }

                        // Emit quad
                        let layer = voxel_id as u32 - 1;
                        let wf = w as f32;
                        let hf = h as f32;

                        // Calculate actual positions based on direction
                        let mut pos = [[0.0; 3]; 4];
                        let dx = offset_x;
                        let dy = offset_y;
                        let dz = offset_z;

                        match dir {
                            0 => { // Top (+Y)
                                let x0 = u as f32 + dx; let z0 = v as f32 + dz; let y0 = d as f32 + dy + 1.0;
                                pos[0] = [x0, y0, z0 + hf];
                                pos[1] = [x0 + wf, y0, z0 + hf];
                                pos[2] = [x0 + wf, y0, z0];
                                pos[3] = [x0, y0, z0];
                            },
                            1 => { // Bottom (-Y)
                                let x0 = u as f32 + dx; let z0 = v as f32 + dz; let y0 = d as f32 + dy;
                                pos[0] = [x0, y0, z0];
                                pos[1] = [x0 + wf, y0, z0];
                                pos[2] = [x0 + wf, y0, z0 + hf];
                                pos[3] = [x0, y0, z0 + hf];
                            },
                            2 => { // Right (+X)
                                let z0 = u as f32 + dz; let y0 = v as f32 + dy; let x0 = d as f32 + dx + 1.0;
                                pos[0] = [x0, y0, z0 + wf];
                                pos[1] = [x0, y0 + hf, z0 + wf];
                                pos[2] = [x0, y0 + hf, z0];
                                pos[3] = [x0, y0, z0];
                            },
                            3 => { // Left (-X)
                                let z0 = u as f32 + dz; let y0 = v as f32 + dy; let x0 = d as f32 + dx;
                                pos[0] = [x0, y0, z0];
                                pos[1] = [x0, y0 + hf, z0];
                                pos[2] = [x0, y0 + hf, z0 + wf];
                                pos[3] = [x0, y0, z0 + wf];
                            },
                            4 => { // Front (+Z)
                                let x0 = u as f32 + dx; let y0 = v as f32 + dy; let z0 = d as f32 + dz + 1.0;
                                pos[0] = [x0, y0, z0];
                                pos[1] = [x0, y0 + hf, z0];
                                pos[2] = [x0 + wf, y0 + hf, z0];
                                pos[3] = [x0 + wf, y0, z0];
                            },
                            5 => { // Back (-Z)
                                let x0 = u as f32 + dx; let y0 = v as f32 + dy; let z0 = d as f32 + dz;
                                pos[0] = [x0 + wf, y0, z0];
                                pos[1] = [x0 + wf, y0 + hf, z0];
                                pos[2] = [x0, y0 + hf, z0];
                                pos[3] = [x0, y0, z0];
                            },
                            _ => unreachable!(),
                        }

                        // Add UVs taking width/height into account
                        // Map face coordinates [BL, BR, TR, TL] -> UVs [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]
                        vertices.push(Vertex { pos: pos[0], uv: [0.0, hf], layer });
                        vertices.push(Vertex { pos: pos[1], uv: [wf, hf], layer });
                        vertices.push(Vertex { pos: pos[2], uv: [wf, 0.0], layer });
                        vertices.push(Vertex { pos: pos[3], uv: [0.0, 0.0], layer });
                        indices.extend_from_slice(&[index_count, index_count+1, index_count+2, index_count+2, index_count+3, index_count]);
                        index_count += 4;

                        u += w;
                    } else {
                        u += 1;
                    }
                }
            }
        }
    }

    Mesh { vertices, indices }
}