use crate::chunk::{Chunk, CHUNK_HEIGHT, CHUNK_SIZE};
use glam::IVec3;
use std::collections::HashMap;

pub struct World {
    pub chunks: HashMap<IVec3, Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(chunk.position, chunk);
    }

    pub fn get_chunk(&self, pos: &IVec3) -> Option<&Chunk> {
        self.chunks.get(pos)
    }

    #[allow(dead_code)]
    pub fn remove_chunk(&mut self, pos: &IVec3) -> Option<Chunk> {
        self.chunks.remove(pos)
    }

    pub fn get_voxel(&self, global_x: i32, global_y: i32, global_z: i32) -> u16 {
        if global_y < 0 || global_y >= CHUNK_HEIGHT as i32 {
            return 0;
        }

        let chunk_x = global_x.div_euclid(CHUNK_SIZE as i32);
        let chunk_z = global_z.div_euclid(CHUNK_SIZE as i32);
        let chunk_pos = IVec3::new(chunk_x, 0, chunk_z);

        if let Some(chunk) = self.chunks.get(&chunk_pos) {
            let local_x = global_x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_y = global_y as usize;
            let local_z = global_z.rem_euclid(CHUNK_SIZE as i32) as usize;
            chunk.get_voxel(local_x, local_y, local_z)
        } else {
            0
        }
    }
}
