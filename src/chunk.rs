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

    #[inline(always)]
    pub fn get_voxel_safe(&self, x: i32, y: i32, z: i32) -> u16 {
        if x < 0 || y < 0 || z < 0 || x >= CHUNK_SIZE as i32 || y >= CHUNK_HEIGHT as i32 || z >= CHUNK_SIZE as i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_new_is_empty() {
        let chunk = Chunk::new(IVec3::ZERO);
        assert_eq!(chunk.get_voxel(0, 0, 0), 0);
        assert_eq!(chunk.get_voxel(10, 100, 10), 0);
        assert_eq!(chunk.get_voxel(31, 255, 31), 0);
    }

    #[test]
    fn test_set_and_get_voxel() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(5, 50, 5, 42);
        assert_eq!(chunk.get_voxel(5, 50, 5), 42);
        assert_eq!(chunk.get_voxel_safe(5, 50, 5), 42);
    }

    #[test]
    fn test_out_of_bounds() {
        let chunk = Chunk::new(IVec3::ZERO);
        // Test get_voxel
        assert_eq!(chunk.get_voxel(32, 0, 0), 0);
        assert_eq!(chunk.get_voxel(0, 256, 0), 0);
        assert_eq!(chunk.get_voxel(0, 0, 32), 0);

        // Test get_voxel_safe
        assert_eq!(chunk.get_voxel_safe(-1, 0, 0), 0);
        assert_eq!(chunk.get_voxel_safe(0, -1, 0), 0);
        assert_eq!(chunk.get_voxel_safe(0, 0, -1), 0);
        assert_eq!(chunk.get_voxel_safe(32, 0, 0), 0);
        assert_eq!(chunk.get_voxel_safe(0, 256, 0), 0);
        assert_eq!(chunk.get_voxel_safe(0, 0, 32), 0);
    }

    #[test]
    fn test_set_out_of_bounds() {
        let mut chunk = Chunk::new(IVec3::ZERO);
        chunk.set_voxel(32, 0, 0, 1);
        chunk.set_voxel(0, 256, 0, 1);
        chunk.set_voxel(0, 0, 32, 1);
        // No crash should happen, and inside should still be 0
        assert_eq!(chunk.get_voxel(31, 0, 0), 0);
        assert_eq!(chunk.get_voxel(0, 255, 0), 0);
        assert_eq!(chunk.get_voxel(0, 0, 31), 0);
    }
}