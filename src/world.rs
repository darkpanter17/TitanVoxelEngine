use crate::chunk::{Chunk, CHUNK_SIZE, CHUNK_HEIGHT};
use std::collections::HashMap;
use glam::IVec3;

pub struct World {
    pub chunks: HashMap<IVec3, Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_chunk(&self, chunk_pos: IVec3) -> Option<&Chunk> {
        self.chunks.get(&chunk_pos)
    }

    #[allow(dead_code)]
    pub fn get_chunk_mut(&mut self, chunk_pos: IVec3) -> Option<&mut Chunk> {
        self.chunks.get_mut(&chunk_pos)
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(chunk.position, chunk);
    }

    /// Obtiene el voxel usando coordenadas globales (mundo)
    #[allow(dead_code)]
    pub fn get_voxel_global(&self, global_x: i32, global_y: i32, global_z: i32) -> u16 {
        if global_y < 0 || global_y >= CHUNK_HEIGHT as i32 {
            return 0; // Fuera del límite vertical del mundo
        }

        let chunk_pos = IVec3::new(
            global_x.div_euclid(CHUNK_SIZE as i32),
            0,
            global_z.div_euclid(CHUNK_SIZE as i32),
        );

        if let Some(chunk) = self.get_chunk(chunk_pos) {
            let local_x = global_x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_y = global_y as usize;
            let local_z = global_z.rem_euclid(CHUNK_SIZE as i32) as usize;
            chunk.get_voxel(local_x, local_y, local_z)
        } else {
            0 // Chunk no cargado
        }
    }

    /// Asigna un voxel usando coordenadas globales (mundo)
    #[allow(dead_code)]
    pub fn set_voxel_global(&mut self, global_x: i32, global_y: i32, global_z: i32, id: u16) {
        if global_y < 0 || global_y >= CHUNK_HEIGHT as i32 {
            return;
        }

        let chunk_pos = IVec3::new(
            global_x.div_euclid(CHUNK_SIZE as i32),
            0,
            global_z.div_euclid(CHUNK_SIZE as i32),
        );

        if let Some(chunk) = self.get_chunk_mut(chunk_pos) {
            let local_x = global_x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_y = global_y as usize;
            let local_z = global_z.rem_euclid(CHUNK_SIZE as i32) as usize;
            chunk.set_voxel(local_x, local_y, local_z, id);
        }
    }
}
