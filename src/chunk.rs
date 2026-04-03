#![allow(dead_code)]

use glam::IVec3;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_HEIGHT: usize = 256;

pub struct Chunk {
    pub position: IVec3,
    // Array plano es 10x más rápido que Vec<Vec<Vec>>>
    pub data: Box<[u16; CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE]>,
}

impl Chunk {
    pub fn new(pos: IVec3) -> Self {
        Self {
            position: pos,
            data: Box::new([0; CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE]),
        }
    }

    /// Índice lineal: x más rápido, luego z, luego y (size = 32×256×32 = 262144).
    #[inline(always)]
    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> u16 {
        if x >= CHUNK_SIZE || y >= CHUNK_HEIGHT || z >= CHUNK_SIZE { return 0; }
        self.data[x + (z * CHUNK_SIZE) + (y * CHUNK_SIZE * CHUNK_SIZE)]
    }

    pub fn get_voxel_safe(&self, x: i32, y: i32, z: i32) -> u16 {
        if x < 0 || x >= CHUNK_SIZE as i32 || y < 0 || y >= CHUNK_HEIGHT as i32 || z < 0 || z >= CHUNK_SIZE as i32 {
            return 0;
        }
        self.get_voxel(x as usize, y as usize, z as usize)
    }

    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, id: u16) {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            self.data[x + (z * CHUNK_SIZE) + (y * CHUNK_SIZE * CHUNK_SIZE)] = id;
        }
    }
}