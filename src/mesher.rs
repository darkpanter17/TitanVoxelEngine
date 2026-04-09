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
#[inline(always)]
fn get_voxel_safe(chunk: &Chunk, x: i32, y: i32, z: i32) -> u16 {
    if x < 0 || x >= CHUNK_SIZE as i32 || y < 0 || y >= CHUNK_HEIGHT as i32 || z < 0 || z >= CHUNK_SIZE as i32 {
        return 0; // Treat out-of-bounds as air for culling
    }
    chunk.get_voxel(x as usize, y as usize, z as usize)
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    let wx = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let wy = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let wz = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                let voxel_id = chunk.get_voxel(x, y, z);
                if voxel_id != 0 {
                    let layer = (voxel_id - 1) as u32;
                    let fx = x as f32 + wx;
                    let fy = y as f32 + wy;
                    let fz = z as f32 + wz;

                    let xi = x as i32;
                    let yi = y as i32;
                    let zi = z as i32;

                    // Y+ (Top) - CCW
                    if get_voxel_safe(chunk, xi, yi + 1, zi) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count, layer,
                            [[fx, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz], [fx, fy + 1.0, fz]]
                        );
                    }
                    // Y- (Bottom) - CCW
                    if get_voxel_safe(chunk, xi, yi - 1, zi) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count, layer,
                            [[fx, fy, fz], [fx + 1.0, fy, fz], [fx + 1.0, fy, fz + 1.0], [fx, fy, fz + 1.0]]
                        );
                    }
                    // X+ (Right) - CCW
                    if get_voxel_safe(chunk, xi + 1, yi, zi) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count, layer,
                            [[fx + 1.0, fy, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz], [fx + 1.0, fy, fz]]
                        );
                    }
                    // X- (Left) - CCW
                    if get_voxel_safe(chunk, xi - 1, yi, zi) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count, layer,
                            [[fx, fy, fz], [fx, fy + 1.0, fz], [fx, fy + 1.0, fz + 1.0], [fx, fy, fz + 1.0]]
                        );
                    }
                    // Z+ (Front) - CCW
                    if get_voxel_safe(chunk, xi, yi, zi + 1) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count, layer,
                            [[fx, fy, fz + 1.0], [fx, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx + 1.0, fy, fz + 1.0]]
                        );
                    }
                    // Z- (Back) - CCW
                    if get_voxel_safe(chunk, xi, yi, zi - 1) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count, layer,
                            [[fx + 1.0, fy, fz], [fx + 1.0, fy + 1.0, fz], [fx, fy + 1.0, fz], [fx, fy, fz]]
                        );
                    }
                }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_quad(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, layer: u32, pos: [[f32; 3]; 4]) {
    verts.push(Vertex { pos: pos[0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: pos[1], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: pos[2], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: pos[3], uv: [1.0, 1.0], layer });
    inds.extend_from_slice(&[*count, *count + 1, *count + 2, *count + 2, *count + 3, *count]);
    *count += 4;
}