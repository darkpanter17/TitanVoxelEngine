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

    pub fn insert_chunk(&mut self, position: IVec3, chunk: Chunk) {
        self.chunks.insert(position, chunk);
    }

    #[allow(dead_code)]
    pub fn get_chunk(&self, position: &IVec3) -> Option<&Chunk> {
        self.chunks.get(position)
    }

    #[allow(dead_code)]
    pub fn get_chunk_mut(&mut self, position: &IVec3) -> Option<&mut Chunk> {
        self.chunks.get_mut(position)
    }
}
