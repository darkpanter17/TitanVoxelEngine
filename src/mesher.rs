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
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
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
        return true;
    }
    chunk.get_voxel(x as usize, y as usize, z as usize) == 0
}

fn push_face(
    verts: &mut Vec<Vertex>,
    inds: &mut Vec<u32>,
    count: &mut u32,
    pos: [[f32; 3]; 4],
    layer: u32,
    w: f32,
    h: f32,
) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, h], layer });
    verts.push(Vertex { pos: pos[1], uv: [w, h], layer });
    verts.push(Vertex { pos: pos[2], uv: [w, 0.0], layer });
    verts.push(Vertex { pos: pos[3], uv: [0.0, 0.0], layer });
    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count + 2, *count + 3, *count]);
    *count += 4;
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;

    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for dir in 0..6 {
        let (dx, dy, dz) = match dir {
            0 => (0, 1, 0),  // Top (+Y)
            1 => (0, -1, 0), // Bottom (-Y)
            2 => (1, 0, 0),  // Right (+X)
            3 => (-1, 0, 0), // Left (-X)
            4 => (0, 0, 1),  // Front (+Z)
            5 => (0, 0, -1), // Back (-Z)
            _ => unreachable!(),
        };

        // For greedy meshing, we sweep over the axes perpendicular to the face normal.
        // Let's define u and v axes depending on the face normal.
        let (u_axis, v_axis) = match dir {
            0 | 1 => (0, 2), // Top/Bottom faces: sweep over X and Z
            2 | 3 => (2, 1), // Right/Left faces: sweep over Z and Y
            4 | 5 => (0, 1), // Front/Back faces: sweep over X and Y
            _ => unreachable!(),
        };

        let u_size = if u_axis == 1 { CHUNK_HEIGHT } else { CHUNK_SIZE };
        let v_size = if v_axis == 1 { CHUNK_HEIGHT } else { CHUNK_SIZE };
        let slice_size = if dir == 0 || dir == 1 { CHUNK_HEIGHT } else { CHUNK_SIZE };

        for slice in 0..slice_size {
            let mut mask = vec![0u16; u_size * v_size];

            // Build mask
            for v in 0..v_size {
                for u in 0..u_size {
                    let (mut x, mut y, mut z) = (0, 0, 0);

                    if u_axis == 0 { x = u; } else if u_axis == 1 { y = u; } else { z = u; }
                    if v_axis == 0 { x = v; } else if v_axis == 1 { y = v; } else { z = v; }

                    // The slicing axis
                    if dir == 0 || dir == 1 { y = slice; }
                    else if dir == 2 || dir == 3 { x = slice; }
                    else { z = slice; }

                    let voxel_id = chunk.get_voxel(x, y, z);

                    if voxel_id != 0 {
                        // Check if the face is exposed
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        let nz = z as i32 + dz;

                        if is_air(chunk, nx, ny, nz) {
                            mask[u + v * u_size] = voxel_id;
                        }
                    }
                }
            }

            // Greedy meshing over the mask
            let mut j = 0;
            while j < v_size {
                let mut i = 0;
                while i < u_size {
                    let voxel_id = mask[i + j * u_size];
                    if voxel_id != 0 {
                        // Compute width (w)
                        let mut w = 1;
                        while i + w < u_size && mask[i + w + j * u_size] == voxel_id {
                            w += 1;
                        }

                        // Compute height (h)
                        let mut h = 1;
                        'outer: while j + h < v_size {
                            for k in 0..w {
                                if mask[i + k + (j + h) * u_size] != voxel_id {
                                    break 'outer;
                                }
                            }
                            h += 1;
                        }

                        // Mark as merged
                        for hh in 0..h {
                            for ww in 0..w {
                                mask[i + ww + (j + hh) * u_size] = 0;
                            }
                        }

                        // Emit quad
                        let layer = voxel_id as u32 - 1;

                        let (mut x, mut y, mut z) = (0, 0, 0);
                        if u_axis == 0 { x = i; } else if u_axis == 1 { y = i; } else { z = i; }
                        if v_axis == 0 { x = j; } else if v_axis == 1 { y = j; } else { z = j; }

                        if dir == 0 || dir == 1 { y = slice; }
                        else if dir == 2 || dir == 3 { x = slice; }
                        else { z = slice; }

                        let xf = x as f32 + offset_x;
                        let yf = y as f32 + offset_y;
                        let zf = z as f32 + offset_z;

                        let wf = w as f32;
                        let hf = h as f32;

                        match dir {
                            0 => { // Top (+Y)
                                push_face(&mut vertices, &mut indices, &mut index_count,
                                    [[xf, yf + 1.0, zf + hf], [xf + wf, yf + 1.0, zf + hf], [xf + wf, yf + 1.0, zf], [xf, yf + 1.0, zf]], layer, wf, hf);
                            }
                            1 => { // Bottom (-Y)
                                push_face(&mut vertices, &mut indices, &mut index_count,
                                    [[xf, yf, zf], [xf + wf, yf, zf], [xf + wf, yf, zf + hf], [xf, yf, zf + hf]], layer, wf, hf);
                            }
                            2 => { // Right (+X)
                                push_face(&mut vertices, &mut indices, &mut index_count,
                                    [[xf + 1.0, yf, zf + wf], [xf + 1.0, yf + hf, zf + wf], [xf + 1.0, yf + hf, zf], [xf + 1.0, yf, zf]], layer, wf, hf);
                            }
                            3 => { // Left (-X)
                                push_face(&mut vertices, &mut indices, &mut index_count,
                                    [[xf, yf, zf], [xf, yf + hf, zf], [xf, yf + hf, zf + wf], [xf, yf, zf + wf]], layer, wf, hf);
                            }
                            4 => { // Front (+Z)
                                push_face(&mut vertices, &mut indices, &mut index_count,
                                    [[xf, yf, zf + 1.0], [xf, yf + hf, zf + 1.0], [xf + wf, yf + hf, zf + 1.0], [xf + wf, yf, zf + 1.0]], layer, wf, hf);
                            }
                            5 => { // Back (-Z)
                                push_face(&mut vertices, &mut indices, &mut index_count,
                                    [[xf + wf, yf, zf], [xf + wf, yf + hf, zf], [xf, yf + hf, zf], [xf, yf, zf]], layer, wf, hf);
                            }
                            _ => unreachable!()
                        }
                    }
                    i += 1;
                }
                j += 1;
            }
        }
    }

    Mesh { vertices, indices }
}