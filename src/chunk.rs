use glam::IVec3;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_HEIGHT: usize = 256;

pub struct Chunk {
    #[allow(dead_code)]
    pub position: IVec3,
    // Flat array is 10x faster than Vec<Vec<Vec>>>
    pub data: Box<[u16; CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE]>,
}

impl Chunk {
    pub fn new(pos: IVec3) -> Self {
        Self {
            position: pos,
            data: Box::new([0; CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE]),
        }
    }

    /// Linear index: x fastest, then z, then y (size = 32×256×32 = 262144).
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
    fn test_chunk_voxel_bounds() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        // Valid set/get
        chunk.set_voxel(0, 0, 0, 1);
        assert_eq!(chunk.get_voxel(0, 0, 0), 1);

        chunk.set_voxel(CHUNK_SIZE - 1, CHUNK_HEIGHT - 1, CHUNK_SIZE - 1, 2);
        assert_eq!(chunk.get_voxel(CHUNK_SIZE - 1, CHUNK_HEIGHT - 1, CHUNK_SIZE - 1), 2);

        // Out of bounds get should return 0 safely
        assert_eq!(chunk.get_voxel(CHUNK_SIZE, 0, 0), 0);
        assert_eq!(chunk.get_voxel(0, CHUNK_HEIGHT, 0), 0);
        assert_eq!(chunk.get_voxel(0, 0, CHUNK_SIZE), 0);
    }
}