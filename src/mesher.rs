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
                wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x2 },
                wgpu::VertexAttribute { offset: 20, shader_location: 2, format: wgpu::VertexFormat::Uint32 },
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

    for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let id = chunk.get_voxel(x, y, z);
                if id == 0 { continue; }

                let layer = (id - 1) as u32;
                let fx = x as f32;
                let fy = y as f32;
                let fz = z as f32;

                // Vertices are passed in CCW order: BL, BR, TR, TL relative to the face normal.

                // Top (Y+)
                if is_transparent(chunk, x as i32, y as i32 + 1, z as i32) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[fx, fy + 1.0, fz + 1.0], [fx + 1.0, fy + 1.0, fz + 1.0],
                        [fx + 1.0, fy + 1.0, fz], [fx, fy + 1.0, fz]],
                        layer
                    );
                }

                // Bottom (Y-)
                if is_transparent(chunk, x as i32, y as i32 - 1, z as i32) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[fx, fy, fz], [fx + 1.0, fy, fz],
                        [fx + 1.0, fy, fz + 1.0], [fx, fy, fz + 1.0]],
                        layer
                    );
                }

                // Left (X-)
                if is_transparent(chunk, x as i32 - 1, y as i32, z as i32) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[fx, fy, fz], [fx, fy, fz + 1.0],
                        [fx, fy + 1.0, fz + 1.0], [fx, fy + 1.0, fz]],
                        layer
                    );
                }

                // Right (X+)
                if is_transparent(chunk, x as i32 + 1, y as i32, z as i32) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[fx + 1.0, fy, fz + 1.0], [fx + 1.0, fy, fz],
                        [fx + 1.0, fy + 1.0, fz], [fx + 1.0, fy + 1.0, fz + 1.0]],
                        layer
                    );
                }

                // Front (Z+)
                if is_transparent(chunk, x as i32, y as i32, z as i32 + 1) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[fx, fy, fz + 1.0], [fx + 1.0, fy, fz + 1.0],
                        [fx + 1.0, fy + 1.0, fz + 1.0], [fx, fy + 1.0, fz + 1.0]],
                        layer
                    );
                }

                // Back (Z-)
                if is_transparent(chunk, x as i32, y as i32, z as i32 - 1) {
                    push_quad(&mut vertices, &mut indices, &mut index_count,
                        [[fx + 1.0, fy, fz], [fx, fy, fz],
                        [fx, fy + 1.0, fz], [fx + 1.0, fy + 1.0, fz]],
                        layer
                    );
                }
            }
        }
    }

    Mesh { vertices, indices }
}

fn is_transparent(chunk: &Chunk, x: i32, y: i32, z: i32) -> bool {
    if x < 0 || y < 0 || z < 0 || x >= CHUNK_SIZE as i32 || y >= CHUNK_HEIGHT as i32 || z >= CHUNK_SIZE as i32 {
        return true;
    }
    chunk.get_voxel(x as usize, y as usize, z as usize) == 0
}

fn push_quad(
    verts: &mut Vec<Vertex>,
    inds: &mut Vec<u32>,
    count: &mut u32,
    pos: [[f32; 3]; 4],
    layer: u32
) {
    // Standard quad UVs
    verts.push(Vertex { pos: pos[0], uv: [0.0, 1.0], layer }); // BL
    verts.push(Vertex { pos: pos[1], uv: [1.0, 1.0], layer }); // BR
    verts.push(Vertex { pos: pos[2], uv: [1.0, 0.0], layer }); // TR
    verts.push(Vertex { pos: pos[3], uv: [0.0, 0.0], layer }); // TL

    // CCW 0 -> 1 -> 2 -> 3 is NOT a triangle fan.
    // Quad is 0,1,2 and 2,3,0
    inds.push(*count);
    inds.push(*count + 1);
    inds.push(*count + 2);

    inds.push(*count + 2);
    inds.push(*count + 3);
    inds.push(*count);

    *count += 4;
}
