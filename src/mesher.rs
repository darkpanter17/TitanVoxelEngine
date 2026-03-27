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
fn get_voxel_safe(chunk: &Chunk, x: isize, y: isize, z: isize) -> u16 {
    if x < 0 || x >= CHUNK_SIZE as isize ||
       y < 0 || y >= CHUNK_HEIGHT as isize ||
       z < 0 || z >= CHUNK_SIZE as isize {
        return 0; // Treat out of bounds as air
    }
    chunk.get_voxel(x as usize, y as usize, z as usize)
}

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                 let voxel = chunk.get_voxel(x, y, z);
                 if voxel != 0 {
                    let layer = voxel as u32 - 1; // 1 -> 0, 2 -> 1, etc.
                    let xf = (chunk.position.x * CHUNK_SIZE as i32 + x as i32) as f32;
                    let yf = (chunk.position.y * CHUNK_HEIGHT as i32 + y as i32) as f32;
                    let zf = (chunk.position.z * CHUNK_SIZE as i32 + z as i32) as f32;
                    let xi = x as isize;
                    let yi = y as isize;
                    let zi = z as isize;

                    // For Top/Bottom faces, UV maps (x, z) to (u, v)
                    // Y+ (Top) - look down Y axis: (-X, -Z) in image? Usually X is U, Z is V
                    // To get correct CCW when looking from +Y:
                    if get_voxel_safe(chunk, xi, yi + 1, zi) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf, yf + 1.0, zf], [xf, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf + 1.0, yf + 1.0, zf]],
                            layer
                        );
                    }
                    // Y- (Bottom) - looking from -Y:
                    if get_voxel_safe(chunk, xi, yi - 1, zi) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf, yf, zf + 1.0], [xf, yf, zf], [xf + 1.0, yf, zf], [xf + 1.0, yf, zf + 1.0]],
                            layer
                        );
                    }
                    // X+ (Right) - looking from +X: Z is U, Y is V
                    if get_voxel_safe(chunk, xi + 1, yi, zi) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf, zf], [xf + 1.0, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf + 1.0]],
                            layer
                        );
                    }
                    // X- (Left) - looking from -X:
                    if get_voxel_safe(chunk, xi - 1, yi, zi) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf, yf, zf], [xf, yf, zf + 1.0], [xf, yf + 1.0, zf + 1.0], [xf, yf + 1.0, zf]],
                            layer
                        );
                    }
                    // Z+ (Front) - looking from +Z: X is U, Y is V
                    if get_voxel_safe(chunk, xi, yi, zi + 1) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf, yf, zf + 1.0], [xf + 1.0, yf, zf + 1.0], [xf + 1.0, yf + 1.0, zf + 1.0], [xf, yf + 1.0, zf + 1.0]],
                            layer
                        );
                    }
                    // Z- (Back) - looking from -Z:
                    if get_voxel_safe(chunk, xi, yi, zi - 1) == 0 {
                        push_quad(
                            &mut vertices, &mut indices, &mut index_count,
                            [[xf + 1.0, yf, zf], [xf, yf, zf], [xf, yf + 1.0, zf], [xf + 1.0, yf + 1.0, zf]],
                            layer
                        );
                    }
                 }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_quad(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, 
             positions: [[f32; 3]; 4], layer: u32) {
    verts.push(Vertex { pos: positions[0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: positions[1], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: positions[2], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: positions[3], uv: [0.0, 0.0], layer });
    // CCW winding: 0, 1, 2, 2, 3, 0
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}