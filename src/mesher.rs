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

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    // Greedy meshing for 6 axes (d): 0=Y+, 1=Y-, 2=X+, 3=X-, 4=Z+, 5=Z-
    for d in 0..6 {
        // En Y (0,1) iteramos por altura, los ejes transversales son X y Z
        // En X (2,3) iteramos por anchura (X), transversales son Z e Y
        // En Z (4,5) iteramos por profundidad (Z), transversales son X e Y
        let (axis_limit, u_limit, v_limit) = match d {
            0 | 1 => (CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_SIZE), // Y faces
            2 | 3 => (CHUNK_SIZE, CHUNK_SIZE, CHUNK_HEIGHT), // X faces
            4 | 5 => (CHUNK_SIZE, CHUNK_SIZE, CHUNK_HEIGHT), // Z faces
            _ => unreachable!(),
        };

        for i in 0..axis_limit {
            let mut mask = vec![0u16; u_limit * v_limit];

            // Build mask
            for v in 0..v_limit {
                for u in 0..u_limit {
                    let (x, y, z) = match d {
                        0 | 1 => (u, i, v), // Y faces: u=X, v=Z
                        2 | 3 => (i, v, u), // X faces: u=Z, v=Y
                        4 | 5 => (u, v, i), // Z faces: u=X, v=Y
                        _ => unreachable!(),
                    };

                    let voxel = chunk.get_voxel(x, y, z);
                    if voxel != 0 {
                        // Check if face is exposed
                        let exposed = match d {
                            0 => is_air(chunk, x as i32, y as i32 + 1, z as i32), // Y+
                            1 => is_air(chunk, x as i32, y as i32 - 1, z as i32), // Y-
                            2 => is_air(chunk, x as i32 + 1, y as i32, z as i32), // X+
                            3 => is_air(chunk, x as i32 - 1, y as i32, z as i32), // X-
                            4 => is_air(chunk, x as i32, y as i32, z as i32 + 1), // Z+
                            5 => is_air(chunk, x as i32, y as i32, z as i32 - 1), // Z-
                            _ => false,
                        };
                        if exposed {
                            mask[v * u_limit + u] = voxel;
                        }
                    }
                }
            }

            // Mesh the mask
            let mut v = 0;
            while v < v_limit {
                let mut u = 0;
                while u < u_limit {
                    let voxel = mask[v * u_limit + u];
                    if voxel != 0 {
                        // Compute width
                        let mut width = 1;
                        while u + width < u_limit && mask[v * u_limit + (u + width)] == voxel {
                            width += 1;
                        }

                        // Compute height
                        let mut height = 1;
                        let mut done = false;
                        while v + height < v_limit {
                            for w in 0..width {
                                if mask[(v + height) * u_limit + (u + w)] != voxel {
                                    done = true;
                                    break;
                                }
                            }
                            if done { break; }
                            height += 1;
                        }

                        // Clear the fused region in the mask
                        for hv in 0..height {
                            for wu in 0..width {
                                mask[(v + hv) * u_limit + (u + wu)] = 0;
                            }
                        }

                        // Generate the quad
                        let layer = voxel as u32 - 1;
                        let wf = width as f32;
                        let hf = height as f32;

                        let (xf, yf, zf) = match d {
                            0 | 1 => (u as f32 + offset_x, i as f32 + offset_y, v as f32 + offset_z), // Y faces: u=X, v=Z
                            2 | 3 => (i as f32 + offset_x, v as f32 + offset_y, u as f32 + offset_z), // X faces: u=Z, v=Y
                            4 | 5 => (u as f32 + offset_x, v as f32 + offset_y, i as f32 + offset_z), // Z faces: u=X, v=Y
                            _ => unreachable!(),
                        };

                        // push_face coordinates: [Bottom-Left, Top-Left, Top-Right, Bottom-Right]
                        // X and Z coordinates use wf (width) and hf (height) appropriately depending on the face mapping.
                        match d {
                            0 => push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf, yf + 1.0, zf + hf], [xf, yf + 1.0, zf], [xf + wf, yf + 1.0, zf], [xf + wf, yf + 1.0, zf + hf]], wf, hf, layer), // Top (+Y)
                            1 => push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf, yf, zf], [xf, yf, zf + hf], [xf + wf, yf, zf + hf], [xf + wf, yf, zf]], wf, hf, layer), // Bottom (-Y)
                            2 => push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf + 1.0, yf, zf + wf], [xf + 1.0, yf + hf, zf + wf], [xf + 1.0, yf + hf, zf], [xf + 1.0, yf, zf]], wf, hf, layer), // Right (+X), u=Z, v=Y
                            3 => push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf, yf, zf], [xf, yf + hf, zf], [xf, yf + hf, zf + wf], [xf, yf, zf + wf]], wf, hf, layer), // Left (-X), u=Z, v=Y
                            4 => push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf + wf, yf, zf + 1.0], [xf + wf, yf + hf, zf + 1.0], [xf, yf + hf, zf + 1.0], [xf, yf, zf + 1.0]], wf, hf, layer), // Front (+Z), u=X, v=Y
                            5 => push_face(&mut vertices, &mut indices, &mut index_count,
                                [[xf, yf, zf], [xf, yf + hf, zf], [xf + wf, yf + hf, zf], [xf + wf, yf, zf]], wf, hf, layer), // Back (-Z), u=X, v=Y
                            _ => {}
                        }
                    }
                    u += 1;
                }
                v += 1;
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             pos: [[f32; 3]; 4], width: f32, height: f32, layer: u32) {
    // Restore the original UV mapping order [BL, TL, TR, BR]
    // Bottom-Left
    verts.push(Vertex { pos: pos[0], uv: [0.0, height], layer });
    // Top-Left
    verts.push(Vertex { pos: pos[1], uv: [0.0, 0.0], layer });
    // Top-Right
    verts.push(Vertex { pos: pos[2], uv: [width, 0.0], layer });
    // Bottom-Right
    verts.push(Vertex { pos: pos[3], uv: [width, height], layer });

    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_face() {
        let mut verts = Vec::new();
        let mut inds = Vec::new();
        let mut count = 0;

        let pos = [
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
        ];

        push_face(&mut verts, &mut inds, &mut count, pos, 1.0, 1.0, 0);

        assert_eq!(verts.len(), 4);
        assert_eq!(inds.len(), 6);
        assert_eq!(count, 4);

        // Verify UVs are correct for [BL, TL, TR, BR] with CCW indices
        assert_eq!(verts[0].uv, [0.0, 1.0]);
        assert_eq!(verts[1].uv, [0.0, 0.0]);
        assert_eq!(verts[2].uv, [1.0, 0.0]);
        assert_eq!(verts[3].uv, [1.0, 1.0]);

        // Verify indices for CCW triangles
        assert_eq!(inds, vec![0, 1, 2, 2, 3, 0]);
    }
}