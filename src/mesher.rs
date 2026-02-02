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
    
    // NOTA: Para este test, simplificamos el loop solo para mostrar geometría rápida
    // En producción, aquí va tu algoritmo completo del Patch 01.
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            for y in 0..100 { // Dibujamos hasta altura 100
                 if chunk.get_voxel(x, y, z) != 0 {
                    // Generar cubo simple si hay voxel (Placeholder para test gráfico)
                    // Cara Superior
                    let xf = x as f32; let yf = y as f32; let zf = z as f32;
                    push_quad(&mut vertices, &mut indices, &mut index_count, xf, yf+1.0, zf, 1.0, 1.0, chunk.get_voxel(x,y,z) as u32);
                 }
            }
        }
    }
    Mesh { vertices, indices }
}

fn push_quad(verts: &mut Vec<Vertex>, inds: &mut Vec<u32>, count: &mut u32, 
             x: f32, y: f32, z: f32, w: f32, d: f32, layer: u32) {
    verts.push(Vertex { pos: [x, y, z], uv: [0.0, 0.0], layer });
    verts.push(Vertex { pos: [x+w, y, z], uv: [w, 0.0], layer });
    verts.push(Vertex { pos: [x+w, y, z+d], uv: [w, d], layer });
    verts.push(Vertex { pos: [x, y, z+d], uv: [0.0, d], layer });
    inds.extend_from_slice(&[*count, *count+1, *count+2, *count+2, *count+3, *count]);
    *count += 4;
}