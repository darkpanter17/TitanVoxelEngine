use glam::IVec3;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_HEIGHT: usize = 256;

pub struct Chunk {
    #[allow(dead_code)]
    pub position: IVec3,
    // Flat array is 10x faster than Vec<Vec<Vec>>
    pub data: Box<[u16; CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE]>,
}

impl Chunk {
    pub fn new(pos: IVec3) -> Self {
        Self {
            position: pos,
            data: Box::new([0; CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE]),
        }
    }

    /// Linear index: x fastest, then z, then y (size = 32x256x32 = 262144).
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
    fn test_chunk_get_set_voxel() {
        let mut chunk = Chunk::new(IVec3::ZERO);

        // Verify default is 0
        assert_eq!(chunk.get_voxel(5, 5, 5), 0);

        // Set and get within bounds
        chunk.set_voxel(5, 5, 5, 1);
        assert_eq!(chunk.get_voxel(5, 5, 5), 1);

        // Verify get_voxel returns 0 out of bounds without panicking
        assert_eq!(chunk.get_voxel(CHUNK_SIZE, 0, 0), 0);
        assert_eq!(chunk.get_voxel(0, CHUNK_HEIGHT, 0), 0);
        assert_eq!(chunk.get_voxel(0, 0, CHUNK_SIZE), 0);

        // Verify set_voxel ignores out of bounds gracefully
        chunk.set_voxel(CHUNK_SIZE, 0, 0, 2); // Should not panic or overwrite anything bad
    }
}