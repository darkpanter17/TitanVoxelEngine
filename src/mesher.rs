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

    // Helper closure to map voxel IDs to layer
    let get_layer = |id: u16| {
        if id == 0 { None } else { Some(id as u32 - 1) }
    };

    // Y Axis (Top & Bottom Faces)
    for y in 0..CHUNK_HEIGHT {
        let mut mask_top = [None; CHUNK_SIZE * CHUNK_SIZE];
        let mut mask_bottom = [None; CHUNK_SIZE * CHUNK_SIZE];
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let id = chunk.get_voxel(x, y, z);
                if id != 0 {
                    let layer = get_layer(id);
                    if is_air(chunk, x as i32, y as i32 + 1, z as i32) {
                        mask_top[x + z * CHUNK_SIZE] = layer;
                    }
                    if is_air(chunk, x as i32, y as i32 - 1, z as i32) {
                        mask_bottom[x + z * CHUNK_SIZE] = layer;
                    }
                }
            }
        }

        greedy_mesh_2d(&mut mask_top, CHUNK_SIZE, CHUNK_SIZE, |x, z, w, h, layer| {
            let xf = x as f32 + offset_x; let yf = y as f32 + offset_y; let zf = z as f32 + offset_z;
            let wf = w as f32; let hf = h as f32;
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf, yf + 1.0, zf + hf], [xf + wf, yf + 1.0, zf + hf], [xf + wf, yf + 1.0, zf], [xf, yf + 1.0, zf]],
                layer, wf, hf);
        });

        greedy_mesh_2d(&mut mask_bottom, CHUNK_SIZE, CHUNK_SIZE, |x, z, w, h, layer| {
            let xf = x as f32 + offset_x; let yf = y as f32 + offset_y; let zf = z as f32 + offset_z;
            let wf = w as f32; let hf = h as f32;
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf, yf, zf], [xf + wf, yf, zf], [xf + wf, yf, zf + hf], [xf, yf, zf + hf]],
                layer, wf, hf);
        });
    }

    // X Axis (Right & Left Faces)
    for x in 0..CHUNK_SIZE {
        let mut mask_right = [None; CHUNK_SIZE * CHUNK_HEIGHT];
        let mut mask_left = [None; CHUNK_SIZE * CHUNK_HEIGHT];
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let id = chunk.get_voxel(x, y, z);
                if id != 0 {
                    let layer = get_layer(id);
                    if is_air(chunk, x as i32 + 1, y as i32, z as i32) {
                        mask_right[z + y * CHUNK_SIZE] = layer;
                    }
                    if is_air(chunk, x as i32 - 1, y as i32, z as i32) {
                        mask_left[z + y * CHUNK_SIZE] = layer;
                    }
                }
            }
        }

        greedy_mesh_2d(&mut mask_right, CHUNK_SIZE, CHUNK_HEIGHT, |z, y, w, h, layer| {
            let xf = x as f32 + offset_x; let yf = y as f32 + offset_y; let zf = z as f32 + offset_z;
            let wf = w as f32; let hf = h as f32;
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf + 1.0, yf, zf + wf], [xf + 1.0, yf + hf, zf + wf], [xf + 1.0, yf + hf, zf], [xf + 1.0, yf, zf]],
                layer, wf, hf);
        });

        greedy_mesh_2d(&mut mask_left, CHUNK_SIZE, CHUNK_HEIGHT, |z, y, w, h, layer| {
            let xf = x as f32 + offset_x; let yf = y as f32 + offset_y; let zf = z as f32 + offset_z;
            let wf = w as f32; let hf = h as f32;
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf, yf, zf], [xf, yf + hf, zf], [xf, yf + hf, zf + wf], [xf, yf, zf + wf]],
                layer, wf, hf);
        });
    }

    // Z Axis (Front & Back Faces)
    for z in 0..CHUNK_SIZE {
        let mut mask_front = [None; CHUNK_SIZE * CHUNK_HEIGHT];
        let mut mask_back = [None; CHUNK_SIZE * CHUNK_HEIGHT];
        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_SIZE {
                let id = chunk.get_voxel(x, y, z);
                if id != 0 {
                    let layer = get_layer(id);
                    if is_air(chunk, x as i32, y as i32, z as i32 + 1) {
                        mask_front[x + y * CHUNK_SIZE] = layer;
                    }
                    if is_air(chunk, x as i32, y as i32, z as i32 - 1) {
                        mask_back[x + y * CHUNK_SIZE] = layer;
                    }
                }
            }
        }

        greedy_mesh_2d(&mut mask_front, CHUNK_SIZE, CHUNK_HEIGHT, |x, y, w, h, layer| {
            let xf = x as f32 + offset_x; let yf = y as f32 + offset_y; let zf = z as f32 + offset_z;
            let wf = w as f32; let hf = h as f32;
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf, yf, zf + 1.0], [xf, yf + hf, zf + 1.0], [xf + wf, yf + hf, zf + 1.0], [xf + wf, yf, zf + 1.0]],
                layer, wf, hf);
        });

        greedy_mesh_2d(&mut mask_back, CHUNK_SIZE, CHUNK_HEIGHT, |x, y, w, h, layer| {
            let xf = x as f32 + offset_x; let yf = y as f32 + offset_y; let zf = z as f32 + offset_z;
            let wf = w as f32; let hf = h as f32;
            push_face(&mut vertices, &mut indices, &mut index_count,
                [[xf + wf, yf, zf], [xf + wf, yf + hf, zf], [xf, yf + hf, zf], [xf, yf, zf]],
                layer, wf, hf);
        });
    }

    Mesh { vertices, indices }
}

fn greedy_mesh_2d<F>(mask: &mut [Option<u32>], width: usize, height: usize, mut build_quad: F)
where
    F: FnMut(usize, usize, usize, usize, u32),
{
    for j in 0..height {
        for i in 0..width {
            if let Some(layer) = mask[i + j * width] {
                // Compute width
                let mut w = 1;
                while i + w < width && mask[i + w + j * width] == Some(layer) {
                    w += 1;
                }

                // Compute height
                let mut h = 1;
                'outer: while j + h < height {
                    for k in 0..w {
                        if mask[i + k + (j + h) * width] != Some(layer) {
                            break 'outer;
                        }
                    }
                    h += 1;
                }

                // Clear mask
                for dy in 0..h {
                    for dx in 0..w {
                        mask[i + dx + (j + dy) * width] = None;
                    }
                }

                build_quad(i, j, w, h, layer);
            }
        }
    }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             pos: [[f32; 3]; 4], layer: u32, w: f32, h: f32) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, h], layer });
    verts.push(Vertex { pos: pos[1], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: pos[2], uv: [w, 0.0], layer });
    verts.push(Vertex { pos: pos[3], uv: [w, h], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}