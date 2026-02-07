use crate::chunk::{Chunk, CHUNK_SIZE, CHUNK_HEIGHT};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub layer: u32,
}

impl Vertex {
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
                wgpu::VertexAttribute { // normal
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { // layer
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
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
                let voxel = chunk.get_voxel(x, y, z);
                if voxel == 0 { continue; }

                let layer = (voxel - 1) as u32;
                let fx = x as f32;
                let fy = y as f32;
                let fz = z as f32;

                // Top (+Y)
                if y == CHUNK_HEIGHT - 1 || chunk.get_voxel(x, y + 1, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx, fy + 1.0, fz],      // Origin (Top-Left corner of top face?) No, (x, y+1, z) is corner 0.
                        [1.0, 0.0, 0.0],         // U direction
                        [0.0, 0.0, 1.0],         // V direction
                        [0.0, 1.0, 0.0], layer); // Normal
                }
                // Bottom (-Y)
                if y == 0 || chunk.get_voxel(x, y - 1, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx, fy, fz + 1.0],
                        [1.0, 0.0, 0.0],
                        [0.0, 0.0, -1.0],
                        [0.0, -1.0, 0.0], layer);
                }

                // Right (+X)
                if x == CHUNK_SIZE - 1 || chunk.get_voxel(x + 1, y, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx + 1.0, fy, fz + 1.0], // Origin: Front-Bottom-Right
                        [0.0, 0.0, -1.0],         // U: Backwards
                        [0.0, 1.0, 0.0],          // V: Up
                        [1.0, 0.0, 0.0], layer);
                }
                // Left (-X)
                if x == 0 || chunk.get_voxel(x - 1, y, z) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx, fy, fz],             // Origin: Front-Bottom-Left
                        [0.0, 0.0, 1.0],          // U: Forwards
                        [0.0, 1.0, 0.0],          // V: Up
                        [-1.0, 0.0, 0.0], layer);
                }

                // Back (+Z)
                if z == CHUNK_SIZE - 1 || chunk.get_voxel(x, y, z + 1) == 0 {
                     push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx + 1.0, fy, fz + 1.0], // Origin: Back-Bottom-Right
                        [-1.0, 0.0, 0.0],         // U: Left
                        [0.0, 1.0, 0.0],          // V: Up
                        [0.0, 0.0, 1.0], layer);
                }
                // Front (-Z)
                if z == 0 || chunk.get_voxel(x, y, z - 1) == 0 {
                    push_face(&mut vertices, &mut indices, &mut index_count,
                        [fx, fy, fz],             // Origin: Front-Bottom-Left
                        [1.0, 0.0, 0.0],          // U: Right
                        [0.0, 1.0, 0.0],          // V: Up
                        [0.0, 0.0, -1.0], layer);
                }
            }
        }
    }

    Mesh { vertices, indices }
}

#[allow(clippy::too_many_arguments)]
fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             origin: [f32; 3], u_dir: [f32; 3], v_dir: [f32; 3], normal: [f32; 3], layer: u32) {

    // 0: origin (UV 0,1)
    // 1: origin + u (UV 1,1)
    // 2: origin + u + v (UV 1,0)
    // 3: origin + v (UV 0,0)

    let p0 = origin;
    let p1 = [origin[0] + u_dir[0], origin[1] + u_dir[1], origin[2] + u_dir[2]];
    let p2 = [origin[0] + u_dir[0] + v_dir[0], origin[1] + u_dir[1] + v_dir[1], origin[2] + u_dir[2] + v_dir[2]];
    let p3 = [origin[0] + v_dir[0], origin[1] + v_dir[1], origin[2] + v_dir[2]];

    // UV coordinates:
    // Assuming (0,0) is top-left of the texture and (1,1) is bottom-right.
    // If V is "Up" in world space, it corresponds to UV V=0 (Top).
    // So Origin (Bottom-Left in world) -> UV (0, 1) (Bottom-Left in texture).

    verts.push(Vertex { pos: p0, uv: [0.0, 1.0], normal, layer });
    verts.push(Vertex { pos: p1, uv: [1.0, 1.0], normal, layer });
    verts.push(Vertex { pos: p2, uv: [1.0, 0.0], normal, layer });
    verts.push(Vertex { pos: p3, uv: [0.0, 0.0], normal, layer });

    // Indices (CCW)
    // 0 -> 1 -> 2
    // 2 -> 3 -> 0
    inds.push(*count);
    inds.push(*count + 1);
    inds.push(*count + 2);

    inds.push(*count + 2);
    inds.push(*count + 3);
    inds.push(*count);

    *count += 4;
}
