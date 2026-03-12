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
pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;
    
    // Calculate global position offset
    let offset_x = (chunk.position.x * CHUNK_SIZE as i32) as f32;
    let offset_y = (chunk.position.y * CHUNK_HEIGHT as i32) as f32;
    let offset_z = (chunk.position.z * CHUNK_SIZE as i32) as f32;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                 let voxel = chunk.get_voxel(x, y, z);
                 if voxel != 0 {
                    // El shader usa un ID de array que empieza en 0. Block 1 -> layer 0
                    let layer = (voxel - 1) as u32;

                    let px = x as f32 + offset_x;
                    let py = y as f32 + offset_y;
                    let pz = z as f32 + offset_z;

                    // Cara Top (Y+)
                    if y == CHUNK_HEIGHT - 1 || chunk.get_voxel(x, y + 1, z) == 0 {
                        push_face_top(&mut vertices, &mut indices, &mut index_count, px, py, pz, layer);
                    }
                    // Cara Bottom (Y-)
                    if y == 0 || chunk.get_voxel(x, y - 1, z) == 0 {
                        push_face_bottom(&mut vertices, &mut indices, &mut index_count, px, py, pz, layer);
                    }
                    // Cara Right (X+)
                    if x == CHUNK_SIZE - 1 || chunk.get_voxel(x + 1, y, z) == 0 {
                        push_face_right(&mut vertices, &mut indices, &mut index_count, px, py, pz, layer);
                    }
                    // Cara Left (X-)
                    if x == 0 || chunk.get_voxel(x - 1, y, z) == 0 {
                        push_face_left(&mut vertices, &mut indices, &mut index_count, px, py, pz, layer);
                    }
                    // Cara Front (Z+)
                    if z == CHUNK_SIZE - 1 || chunk.get_voxel(x, y, z + 1) == 0 {
                        push_face_front(&mut vertices, &mut indices, &mut index_count, px, py, pz, layer);
                    }
                    // Cara Back (Z-)
                    if z == 0 || chunk.get_voxel(x, y, z - 1) == 0 {
                        push_face_back(&mut vertices, &mut indices, &mut index_count, px, py, pz, layer);
                    }
                 }
            }
        }
    }
    Mesh { vertices, indices }
}

// Direcciones están en sentido antihorario (CCW) viendo la cara de frente, para que funcionen con CullMode::Back.
// Nota: wgpu espera CCW para frentes por defecto (FrontFace::Ccw).

fn push_face_top(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, x: f32, y: f32, z: f32, layer: u32) {
    verts.push(Vertex { pos: [x, y + 1.0, z], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: [x, y + 1.0, z + 1.0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y + 1.0, z + 1.0], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y + 1.0, z], uv: [1.0, 0.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}

fn push_face_bottom(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, x: f32, y: f32, z: f32, layer: u32) {
    verts.push(Vertex { pos: [x, y, z + 1.0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: [x, y, z], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y, z], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y, z + 1.0], uv: [1.0, 1.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}

fn push_face_right(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, x: f32, y: f32, z: f32, layer: u32) {
    verts.push(Vertex { pos: [x + 1.0, y, z + 1.0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y, z], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y + 1.0, z], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y + 1.0, z + 1.0], uv: [0.0, 0.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}

fn push_face_left(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, x: f32, y: f32, z: f32, layer: u32) {
    verts.push(Vertex { pos: [x, y, z], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: [x, y, z + 1.0], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: [x, y + 1.0, z + 1.0], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: [x, y + 1.0, z], uv: [0.0, 0.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}

fn push_face_front(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, x: f32, y: f32, z: f32, layer: u32) {
    verts.push(Vertex { pos: [x, y, z + 1.0], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y, z + 1.0], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y + 1.0, z + 1.0], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: [x, y + 1.0, z + 1.0], uv: [0.0, 0.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}

fn push_face_back(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, x: f32, y: f32, z: f32, layer: u32) {
    verts.push(Vertex { pos: [x + 1.0, y, z], uv: [0.0, 1.0], layer });
    verts.push(Vertex { pos: [x, y, z], uv: [1.0, 1.0], layer });
    verts.push(Vertex { pos: [x, y + 1.0, z], uv: [1.0, 0.0], layer });
    verts.push(Vertex { pos: [x + 1.0, y + 1.0, z], uv: [0.0, 0.0], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}