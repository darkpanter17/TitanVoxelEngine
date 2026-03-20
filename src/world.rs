use crate::chunk::{Chunk, CHUNK_SIZE};
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

    pub fn generate_terrain(&mut self) {
        // Generate a 3x3 grid of chunks
        for cx in -1..=1 {
            for cz in -1..=1 {
                let chunk_pos = IVec3::new(cx, 0, cz);
                let mut chunk = Chunk::new(chunk_pos);

                for x in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        // Global coordinates for sine wave calculation
                        let global_x = (cx as f32 * CHUNK_SIZE as f32) + x as f32;
                        let global_z = (cz as f32 * CHUNK_SIZE as f32) + z as f32;

                        // Sine wave height mapping
                        let height_offset = (global_x * 0.1).sin() * 5.0 + (global_z * 0.1).cos() * 5.0;
                        let max_y = (15.0 + height_offset) as usize;

                        for y in 0..=max_y {
                            let id = if y == max_y { 1 } else { 2 }; // 1=Grass (Top), 2=Dirt (Bottom)
                            chunk.set_voxel(x, y, z, id);
                        }
                    }
                }
                self.chunks.insert(chunk_pos, chunk);
            }
        }
    }
}
