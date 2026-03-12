use crate::chunk::Chunk;
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

    #[allow(dead_code)]
    pub fn get_chunk(&self, pos: &IVec3) -> Option<&Chunk> {
        self.chunks.get(pos)
    }

    #[allow(dead_code)]
    pub fn get_chunk_mut(&mut self, pos: &IVec3) -> Option<&mut Chunk> {
        self.chunks.get_mut(pos)
    }

    pub fn set_chunk(&mut self, pos: IVec3, chunk: Chunk) {
        self.chunks.insert(pos, chunk);
    }
}
