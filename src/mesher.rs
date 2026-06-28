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

    // We'll iterate through 6 main axes.
    // axis 0 = X, axis 1 = Y, axis 2 = Z
    for axis in 0..3 {
        // Two directions per axis: -1 and +1
        for dir in [0, 1] { // 0 represents -1 direction, 1 represents +1 direction
            let is_positive = dir == 1;
            let dir_sign = if is_positive { 1 } else { -1 };

            // Let's figure out the dimensions to iterate over
            // u and v are the axes perpendicular to 'axis'
            let u_axis = (axis + 1) % 3;
            let v_axis = (axis + 2) % 3;

            let mut dims = [CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE];
            dims[1] = CHUNK_HEIGHT;

            let u_dim = dims[u_axis];
            let v_dim = dims[v_axis];
            let axis_dim = dims[axis];

            // For a specific slice (axis_val) we evaluate all (u, v)
            for axis_val in 0..axis_dim {
                // mask holds the voxel ID for voxels that need a face in this slice/direction
                let mut mask = vec![0u16; u_dim * v_dim];

                for v in 0..v_dim {
                    for u in 0..u_dim {
                        let mut pos = [0; 3];
                        pos[axis] = axis_val;
                        pos[u_axis] = u;
                        pos[v_axis] = v;

                        let current_voxel = chunk.get_voxel(pos[0], pos[1], pos[2]);

                        if current_voxel != 0 {
                            let mut neighbor_pos = [pos[0] as i32, pos[1] as i32, pos[2] as i32];
                            neighbor_pos[axis] += dir_sign;

                            if is_air(chunk, neighbor_pos[0], neighbor_pos[1], neighbor_pos[2]) {
                                mask[u + v * u_dim] = current_voxel;
                            }
                        }
                    }
                }

                // Now run greedy meshing on the mask
                let mut v = 0;
                while v < v_dim {
                    let mut u = 0;
                    while u < u_dim {
                        let voxel_id = mask[u + v * u_dim];
                        if voxel_id != 0 {
                            // Find width (along u)
                            let mut width = 1;
                            while u + width < u_dim && mask[u + width + v * u_dim] == voxel_id {
                                width += 1;
                            }

                            // Find height (along v)
                            let mut height = 1;
                            let mut done = false;
                            while v + height < v_dim && !done {
                                for w in 0..width {
                                    if mask[u + w + (v + height) * u_dim] != voxel_id {
                                        done = true;
                                        break;
                                    }
                                }
                                if !done {
                                    height += 1;
                                }
                            }

                            // Emit quad
                            let layer = voxel_id as u32 - 1;

                            let mut du = [0.0; 3];
                            du[u_axis] = width as f32;

                            let mut dv = [0.0; 3];
                            dv[v_axis] = height as f32;

                            let mut base_pos = [0.0; 3];
                            base_pos[axis] = axis_val as f32 + if is_positive { 1.0 } else { 0.0 };
                            base_pos[u_axis] = u as f32;
                            base_pos[v_axis] = v as f32;

                            base_pos[0] += offset_x;
                            base_pos[1] += offset_y;
                            base_pos[2] += offset_z;

                            // Calculate 4 corners
                            let p0 = [base_pos[0], base_pos[1], base_pos[2]];
                            let p1 = [base_pos[0] + du[0], base_pos[1] + du[1], base_pos[2] + du[2]];
                            let p2 = [base_pos[0] + du[0] + dv[0], base_pos[1] + du[1] + dv[1], base_pos[2] + du[2] + dv[2]];
                            let p3 = [base_pos[0] + dv[0], base_pos[1] + dv[1], base_pos[2] + dv[2]];

                            let width_f = width as f32;
                            let height_f = height as f32;

                            // To maintain CCW winding, we might need to swap corners depending on axis and dir
                            let mut quad = [p0, p1, p2, p3];

                            // Setup default CCW quad mapping based on normal
                            // Need to be careful with normal directions and winding order
                            // Let's construct normals:
                            // axis 0 (+X: is_pos=true), (-X: is_pos=false)
                            // axis 1 (+Y: is_pos=true), (-Y: is_pos=false)
                            // axis 2 (+Z: is_pos=true), (-Z: is_pos=false)

                            // Adjusting corners and UVs to ensure CCW face outward
                            let mut uvs = [[0.0, height_f], [width_f, height_f], [width_f, 0.0], [0.0, 0.0]];

                            match (axis, is_positive) {
                                (0, true) => { // +X (Right)
                                    quad = [p0, p3, p2, p1];
                                    uvs = [[0.0, height_f], [width_f, height_f], [width_f, 0.0], [0.0, 0.0]];
                                }
                                (0, false) => { // -X (Left)
                                    quad = [p0, p1, p2, p3];
                                    uvs = [[0.0, height_f], [width_f, height_f], [width_f, 0.0], [0.0, 0.0]];
                                }
                                (1, true) => { // +Y (Top)
                                    quad = [p0, p1, p2, p3];
                                    uvs = [[0.0, height_f], [width_f, height_f], [width_f, 0.0], [0.0, 0.0]];
                                }
                                (1, false) => { // -Y (Bottom)
                                    quad = [p0, p3, p2, p1];
                                    uvs = [[0.0, height_f], [width_f, height_f], [width_f, 0.0], [0.0, 0.0]];
                                }
                                (2, true) => { // +Z (Front)
                                    quad = [p0, p1, p2, p3];
                                    uvs = [[0.0, height_f], [width_f, height_f], [width_f, 0.0], [0.0, 0.0]];
                                }
                                (2, false) => { // -Z (Back)
                                    quad = [p0, p3, p2, p1];
                                    uvs = [[0.0, height_f], [width_f, height_f], [width_f, 0.0], [0.0, 0.0]];
                                }
                                _ => {}
                            }

                            push_face(&mut vertices, &mut indices, &mut index_count, quad, uvs, layer);

                            // Clear mask
                            for w in 0..width {
                                for h in 0..height {
                                    mask[u + w + (v + h) * u_dim] = 0;
                                }
                            }

                            u += width;
                        } else {
                            u += 1;
                        }
                    }
                    v += 1;
                }
            }
        }
    }

    Mesh { vertices, indices }
}

fn push_face(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32,
             pos: [[f32; 3]; 4], uvs: [[f32; 2]; 4], layer: u32) {
    verts.push(Vertex { pos: pos[0], uv: uvs[0], layer });
    verts.push(Vertex { pos: pos[1], uv: uvs[1], layer });
    verts.push(Vertex { pos: pos[2], uv: uvs[2], layer });
    verts.push(Vertex { pos: pos[3], uv: uvs[3], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}