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
}
