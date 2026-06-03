#pragma once

#include <cstdint>
#include "voxel/VoxelData.hpp"

namespace lh::voxel {

// Procedural terrain generator. Uses deterministic fractal value noise so the
// same seed always produces the same world (no external assets required).
class TerrainGenerator {
public:
    explicit TerrainGenerator(std::uint32_t seed = 1337u) : seed_(seed) {}

    // Fill the given chunk with voxels. The chunk's `coord` must be set to its
    // position in chunk units before calling.
    void generate(Chunk& chunk) const;

    [[nodiscard]] std::uint32_t seed() const { return seed_; }

    // Surface height (in world voxels) at a given world column. Exposed so the
    // engine can place the camera relative to the terrain.
    [[nodiscard]] float height_at(int world_x, int world_z) const;

private:
    [[nodiscard]] float value_noise(float x, float z) const;
    [[nodiscard]] float fractal_noise(float x, float z) const;

    std::uint32_t seed_;
};

} // namespace lh::voxel
