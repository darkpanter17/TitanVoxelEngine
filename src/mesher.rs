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
    chunk.get_voxel_safe(x, y, z) == 0
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;

    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    // 0: +Y (Top), 1: -Y (Bottom), 2: +X (Right), 3: -X (Left), 4: +Z (Front), 5: -Z (Back)
    for d in 0..6 {
        // Para cada eje (Y, X, Z) iteramos capa por capa
        let (u_max, v_max, slice_max) = match d {
            0 | 1 => (CHUNK_SIZE, CHUNK_SIZE, CHUNK_HEIGHT), // Y axis slices (Top/Bottom)
            2 | 3 => (CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SIZE), // X axis slices (Right/Left)
            4 | 5 => (CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SIZE), // Z axis slices (Front/Back)
            _ => unreachable!(),
        };

        for slice in 0..slice_max {
            let mut mask = vec![0u32; u_max * v_max];

            // 1. Calcular máscara 2D para esta capa y dirección
            for v in 0..v_max {
                for u in 0..u_max {
                    let (x, y, z) = match d {
                        0 | 1 => (u, slice, v),
                        2 | 3 => (slice, v, u),
                        4 | 5 => (u, v, slice),
                        _ => unreachable!(),
                    };

                    let voxel = chunk.get_voxel_safe(x as i32, y as i32, z as i32);
                    if voxel != 0 {
                        let (nx, ny, nz) = match d {
                            0 => (x as i32, y as i32 + 1, z as i32),
                            1 => (x as i32, y as i32 - 1, z as i32),
                            2 => (x as i32 + 1, y as i32, z as i32),
                            3 => (x as i32 - 1, y as i32, z as i32),
                            4 => (x as i32, y as i32, z as i32 + 1),
                            5 => (x as i32, y as i32, z as i32 - 1),
                            _ => unreachable!(),
                        };

                        if is_air(chunk, nx, ny, nz) {
                            mask[u + v * u_max] = voxel as u32;
                        }
                    }
                }
            }

            // 2. Greedy Meshing: fusionar caras adyacentes del mismo material
            let mut j = 0;
            while j < v_max {
                let mut i = 0;
                while i < u_max {
                    let voxel = mask[i + j * u_max];
                    if voxel != 0 {
                        // Encontrar anchura (w)
                        let mut w = 1;
                        while i + w < u_max && mask[i + w + j * u_max] == voxel {
                            w += 1;
                        }

                        // Encontrar altura (h)
                        let mut h = 1;
                        'outer: while j + h < v_max {
                            for k in 0..w {
                                if mask[i + k + (j + h) * u_max] != voxel {
                                    break 'outer;
                                }
                            }
                            h += 1;
                        }

                        // Marcar región como procesada (0)
                        for y_idx in 0..h {
                            for x_idx in 0..w {
                                mask[i + x_idx + (j + y_idx) * u_max] = 0;
                            }
                        }

                        let layer = voxel - 1; // Block ID 1 -> layer 0

                        // Coordenadas base en world space
                        let (wx, wy, wz) = match d {
                            0 | 1 => (i as f32 + offset_x, slice as f32 + offset_y, j as f32 + offset_z),
                            2 | 3 => (slice as f32 + offset_x, j as f32 + offset_y, i as f32 + offset_z),
                            4 | 5 => (i as f32 + offset_x, j as f32 + offset_y, slice as f32 + offset_z),
                            _ => unreachable!(),
                        };

                        let wf = w as f32;
                        let hf = h as f32;

                        let pos = match d {
                            0 => [ // +Y (Top)
                                [wx, wy + 1.0, wz + hf],
                                [wx + wf, wy + 1.0, wz + hf],
                                [wx + wf, wy + 1.0, wz],
                                [wx, wy + 1.0, wz]
                            ],
                            1 => [ // -Y (Bottom)
                                [wx, wy, wz],
                                [wx + wf, wy, wz],
                                [wx + wf, wy, wz + hf],
                                [wx, wy, wz + hf]
                            ],
                            2 => [ // +X (Right)
                                [wx + 1.0, wy, wz + wf],
                                [wx + 1.0, wy + hf, wz + wf],
                                [wx + 1.0, wy + hf, wz],
                                [wx + 1.0, wy, wz]
                            ],
                            3 => [ // -X (Left)
                                [wx, wy, wz],
                                [wx, wy + hf, wz],
                                [wx, wy + hf, wz + wf],
                                [wx, wy, wz + wf]
                            ],
                            4 => [ // +Z (Front)
                                [wx, wy, wz + 1.0],
                                [wx, wy + hf, wz + 1.0],
                                [wx + wf, wy + hf, wz + 1.0],
                                [wx + wf, wy, wz + 1.0]
                            ],
                            5 => [ // -Z (Back)
                                [wx + wf, wy, wz],
                                [wx + wf, wy + hf, wz],
                                [wx, wy + hf, wz],
                                [wx, wy, wz]
                            ],
                            _ => unreachable!(),
                        };

                        // Push CCW face con repetido UVs basados en w, h
                        push_face(&mut vertices, &mut indices, &mut index_count, pos, layer, wf, hf);
                    }
                    i += 1;
                }
                j += 1;
            }
        }
    }

    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             pos: [[f32; 3]; 4], layer: u32, w: f32, h: f32) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, h], layer });
    verts.push(Vertex { pos: pos[1], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: pos[2], uv: [w, 0.0], layer });
    verts.push(Vertex { pos: pos[3], uv: [w, h], layer });

    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count + 2, *count + 3, *count]);
    *count += 4;
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_greedy_meshing() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        // Generar un bloque 2x2x1
        chunk.set_voxel(0, 0, 0, 1);
        chunk.set_voxel(1, 0, 0, 1);
        chunk.set_voxel(0, 0, 1, 1);
        chunk.set_voxel(1, 0, 1, 1);

        let mesh = generate_mesh(&chunk);

        // El top, bottom deben ser 1 quad (4 vertices, 6 indices). Total de quads esperado = 1(T)+1(B)+2(F)+2(B)+2(L)+2(R) = 10 quads (40 vertices)
        // Ya que la altura es 1, las paredes laterales no se pueden unir en Y (son de alto 1), pero sí en X o Z.
        // Left (-X): en x=0, hay 2 bloques (z=0 y z=1). Es 1 quad.
        // Right (+X): en x=1, hay 2 bloques (z=0 y z=1). Es 1 quad.
        // Front (+Z): en z=1, hay 2 bloques (x=0 y x=1). Es 1 quad.
        // Back (-Z): en z=0, hay 2 bloques (x=0 y x=1). Es 1 quad.
        // Total quads: T(1) + B(1) + L(1) + R(1) + F(1) + Bk(1) = 6 quads.
        // 6 quads * 4 = 24 vertices.
        assert_eq!(mesh.vertices.len(), 24);
        assert_eq!(mesh.indices.len(), 36);
    }
}