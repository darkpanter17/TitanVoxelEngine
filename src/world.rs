use std::collections::HashMap;
use glam::IVec3;
use crate::chunk::Chunk;

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
    pub fn generate_chunks(&mut self, radius: i32) {
        for x in -radius..=radius {
            for z in -radius..=radius {
                let pos = IVec3::new(x, 0, z);
                let chunk = Chunk::new(pos);

                // We'll leave terrain generation here to state.rs or update it in state.rs.
                // For now, world manages chunks.
                self.chunks.insert(pos, chunk);
            }
        }
    }
}
