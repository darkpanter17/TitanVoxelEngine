use glam::IVec3;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_HEIGHT: usize = 256;

pub struct Chunk {
    #[allow(dead_code)]
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

    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, id: u16) {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            self.data[x + (z * CHUNK_SIZE) + (y * CHUNK_SIZE * CHUNK_SIZE)] = id;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_new_and_get() {
        let chunk = Chunk::new(IVec3::ZERO);
        assert_eq!(chunk.get_voxel(0, 0, 0), 0);
        assert_eq!(chunk.get_voxel(15, 128, 15), 0);
    }

    #[test]
    fn test_chunk_set_and_get() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(5, 10, 15, 42);
        assert_eq!(chunk.get_voxel(5, 10, 15), 42);
    }

    #[test]
    fn test_chunk_out_of_bounds() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(CHUNK_SIZE, 0, 0, 99); // Should ignore
        assert_eq!(chunk.get_voxel(CHUNK_SIZE, 0, 0), 0); // Should return 0
    }
}