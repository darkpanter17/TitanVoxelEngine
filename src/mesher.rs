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

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_count = 0;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let voxel_id = chunk.get_voxel(x, y, z);
                if voxel_id == 0 { continue; }

                // Map BlockID to TextureLayer (0-based)
                let layer = (voxel_id - 1) as u32;

                let fx = x as f32;
                let fy = y as f32;
                let fz = z as f32;

                // Check neighbors

                // LEFT (x-1)
                if x == 0 || chunk.get_voxel(x - 1, y, z) == 0 {
                    // Left face: x=0. Quad: (0,0,0), (0,1,0), (0,1,1), (0,0,1)
                    // Order: BL, TL, TR, BR
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx, fy, fz], [fx, fy + 1.0, fz], [fx, fy + 1.0, fz + 1.0], [fx, fy, fz + 1.0],
                        layer);
                }

                // RIGHT (x+1)
                if x == CHUNK_SIZE - 1 || chunk.get_voxel(x + 1, y, z) == 0 {
                    // Right face: x=1. Quad: (1,0,1), (1,1,1), (1,1,0), (1,0,0)
                    // Order: BL, TL, TR, BR
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx + 1.0, fy, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz], [fx + 1.0, fy, fz],
                        layer);
                }

                // BOTTOM (y-1)
                if y == 0 || chunk.get_voxel(x, y - 1, z) == 0 {
                    // Bottom face: y=0. Quad: (0,0,1), (0,0,0), (1,0,0), (1,0,1)
                    // Order: BL, TL, TR, BR (from below)
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx, fy, fz + 1.0], [fx, fy, fz], [fx + 1.0, fy, fz], [fx + 1.0, fy, fz + 1.0],
                        layer);
                }

                // TOP (y+1)
                if y == CHUNK_HEIGHT - 1 || chunk.get_voxel(x, y + 1, z) == 0 {
                    // Top face: y=1. Quad: (0,1,0), (0,1,1), (1,1,1), (1,1,0)
                    // Order: BL, TL, TR, BR
                     push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx, fy + 1.0, fz], [fx, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz],
                        layer);
                }

                // BACK (z-1)
                if z == 0 || chunk.get_voxel(x, y, z - 1) == 0 {
                    // Back face: z=0. Quad: (1,0,0), (1,1,0), (0,1,0), (0,0,0)
                    // Order: BL, TL, TR, BR
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx + 1.0, fy, fz], [fx + 1.0, fy + 1.0, fz], [fx, fy + 1.0, fz], [fx, fy, fz],
                        layer);
                }

                // FRONT (z+1)
                if z == CHUNK_SIZE - 1 || chunk.get_voxel(x, y, z + 1) == 0 {
                    // Front face: z=1. Quad: (0,0,1), (0,1,1), (1,1,1), (1,0,1)
                    // Order: BL, TL, TR, BR
                     push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx, fy, fz + 1.0], [fx, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0], [fx + 1.0, fy, fz + 1.0],
                        layer);
                }
            }
        }
    }

    Mesh { vertices, indices }
}

fn push_face(
    verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
    v0: [f32; 3], v1: [f32; 3], v2: [f32; 3], v3: [f32; 3],
    layer: u32
) {
    verts.push(Vertex { pos: v0, uv: [0.0, 0.0], layer }); // Bottom-Left
    verts.push(Vertex { pos: v1, uv: [0.0, 1.0], layer }); // Top-Left
    verts.push(Vertex { pos: v2, uv: [1.0, 1.0], layer }); // Top-Right
    verts.push(Vertex { pos: v3, uv: [1.0, 0.0], layer }); // Bottom-Right

    inds.push(*count);
    inds.push(*count + 2); // 0 -> 2 -> 1 (BL -> TR -> TL) => CCW
    inds.push(*count + 1);

    inds.push(*count);
    inds.push(*count + 3); // 0 -> 3 -> 2 (BL -> BR -> TR) => CCW
    inds.push(*count + 2);

    *count += 4;
}
