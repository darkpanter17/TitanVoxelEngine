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
    width: f32,
    height: f32,
) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, height], layer });
    verts.push(Vertex { pos: pos[1], uv: [width, height], layer });
    verts.push(Vertex { pos: pos[2], uv: [width, 0.0], layer });
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

    for face in 0..6 {
        let (dir_x, dir_y, dir_z) = match face {
            0 => (0, 1, 0),  // Top (+Y)
            1 => (0, -1, 0), // Bottom (-Y)
            2 => (1, 0, 0),  // Right (+X)
            3 => (-1, 0, 0), // Left (-X)
            4 => (0, 0, 1),  // Front (+Z)
            5 => (0, 0, -1), // Back (-Z)
            _ => unreachable!(),
        };

        // Determine the limits for our 2D slice based on face direction.
        // width and height limits for the 2D loops.
        let (limit_w, limit_h, limit_d) = match face {
            0 | 1 => (CHUNK_SIZE, CHUNK_SIZE, CHUNK_HEIGHT), // Y faces: sweep across X and Z.
            2 | 3 => (CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SIZE), // X faces: sweep across Z and Y.
            4 | 5 => (CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SIZE), // Z faces: sweep across X and Y.
            _ => unreachable!(),
        };

        for d in 0..limit_d {
            let mut mask = vec![0u16; limit_w * limit_h];
            let mut layer_mask = vec![0u32; limit_w * limit_h];

            // Build the mask
            for h in 0..limit_h {
                for w in 0..limit_w {
                    let (x, y, z) = match face {
                        0 | 1 => (w, d, h), // w=X, h=Z, d=Y
                        2 | 3 => (d, h, w), // w=Z, h=Y, d=X
                        4 | 5 => (w, h, d), // w=X, h=Y, d=Z
                        _ => unreachable!(),
                    };

                    let voxel = chunk.get_voxel(x, y, z);
                    if voxel != 0 {
                        // Check if face is exposed
                        if is_air(chunk, x as i32 + dir_x, y as i32 + dir_y, z as i32 + dir_z) {
                            mask[w + h * limit_w] = 1;
                            layer_mask[w + h * limit_w] = (voxel - 1) as u32;
                        }
                    }
                }
            }

            // Greedy meshing
            let mut h = 0;
            while h < limit_h {
                let mut w = 0;
                while w < limit_w {
                    if mask[w + h * limit_w] != 0 {
                        let layer = layer_mask[w + h * limit_w];

                        // Compute width
                        let mut quad_w = 1;
                        while w + quad_w < limit_w
                            && mask[w + quad_w + h * limit_w] != 0
                            && layer_mask[w + quad_w + h * limit_w] == layer
                        {
                            quad_w += 1;
                        }

                        // Compute height
                        let mut quad_h = 1;
                        'height_loop: while h + quad_h < limit_h {
                            for cw in 0..quad_w {
                                if mask[w + cw + (h + quad_h) * limit_w] == 0
                                    || layer_mask[w + cw + (h + quad_h) * limit_w] != layer
                                {
                                    break 'height_loop;
                                }
                            }
                            quad_h += 1;
                        }

                        // Clear the mask for the found quad
                        for ch in 0..quad_h {
                            for cw in 0..quad_w {
                                mask[w + cw + (h + ch) * limit_w] = 0;
                            }
                        }

                        // Map 2D quad back to 3D world space
                        let dw = quad_w as f32;
                        let dh = quad_h as f32;
                        let fw = w as f32;
                        let fh = h as f32;
                        let fd = d as f32;

                        let p = match face {
                            0 => [ // Top (+Y)
                                [fw, fd + 1.0, fh + dh],
                                [fw + dw, fd + 1.0, fh + dh],
                                [fw + dw, fd + 1.0, fh],
                                [fw, fd + 1.0, fh],
                            ],
                            1 => [ // Bottom (-Y)
                                [fw, fd, fh],
                                [fw + dw, fd, fh],
                                [fw + dw, fd, fh + dh],
                                [fw, fd, fh + dh],
                            ],
                            2 => [ // Right (+X)
                                [fd + 1.0, fh, fw + dw],
                                [fd + 1.0, fh + dh, fw + dw],
                                [fd + 1.0, fh + dh, fw],
                                [fd + 1.0, fh, fw],
                            ],
                            3 => [ // Left (-X)
                                [fd, fh, fw],
                                [fd, fh + dh, fw],
                                [fd, fh + dh, fw + dw],
                                [fd, fh, fw + dw],
                            ],
                            4 => [ // Front (+Z)
                                [fw, fh, fd + 1.0],
                                [fw, fh + dh, fd + 1.0],
                                [fw + dw, fh + dh, fd + 1.0],
                                [fw + dw, fh, fd + 1.0],
                            ],
                            5 => [ // Back (-Z)
                                [fw + dw, fh, fd],
                                [fw + dw, fh + dh, fd],
                                [fw, fh + dh, fd],
                                [fw, fh, fd],
                            ],
                            _ => unreachable!(),
                        };

                        // Apply world offset
                        let p_offset = [
                            [p[0][0] + offset_x, p[0][1] + offset_y, p[0][2] + offset_z],
                            [p[1][0] + offset_x, p[1][1] + offset_y, p[1][2] + offset_z],
                            [p[2][0] + offset_x, p[2][1] + offset_y, p[2][2] + offset_z],
                            [p[3][0] + offset_x, p[3][1] + offset_y, p[3][2] + offset_z],
                        ];

                        push_face(&mut vertices, &mut indices, &mut index_count, p_offset, layer, dw, dh);

                        w += quad_w;
                    } else {
                        w += 1;
                    }
                }
                h += 1;
            }
        }
    }

    Mesh { vertices, indices }
}
