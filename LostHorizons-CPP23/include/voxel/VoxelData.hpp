#pragma once

#include <array>
#include <cstdint>
#include <glm/glm.hpp>

namespace lh::voxel {

// Number of voxels along each axis of a chunk. A chunk is a cube.
inline constexpr int kChunkSize = 32;
inline constexpr int kChunkVolume = kChunkSize * kChunkSize * kChunkSize;

// Material identifiers for a voxel. Air is empty space (not meshed).
enum class VoxelType : std::uint8_t {
    Air = 0,
    Stone,
    Dirt,
    Grass,
    Sand,
    Water,
    Snow,
    Count
};

[[nodiscard]] inline bool is_solid(VoxelType type) {
    return type != VoxelType::Air;
}

// Base albedo color per material, used by the CPU mesher to bake per-vertex
// colors (the v0.1.0 pipeline has no texture system yet).
[[nodiscard]] inline glm::vec3 voxel_color(VoxelType type) {
    switch (type) {
        case VoxelType::Stone: return {0.50f, 0.50f, 0.52f};
        case VoxelType::Dirt:  return {0.45f, 0.31f, 0.18f};
        case VoxelType::Grass: return {0.32f, 0.55f, 0.22f};
        case VoxelType::Sand:  return {0.80f, 0.72f, 0.45f};
        case VoxelType::Water: return {0.20f, 0.40f, 0.70f};
        case VoxelType::Snow:  return {0.92f, 0.95f, 0.98f};
        default:               return {1.0f, 0.0f, 1.0f};
    }
}

// Dense voxel storage for a single chunk. Indexing is x-major then y then z.
class Chunk {
public:
    [[nodiscard]] static int index(int x, int y, int z) {
        return x + kChunkSize * (y + kChunkSize * z);
    }

    [[nodiscard]] VoxelType at(int x, int y, int z) const {
        return voxels_[index(x, y, z)];
    }

    void set(int x, int y, int z, VoxelType type) {
        voxels_[index(x, y, z)] = type;
    }

    [[nodiscard]] static bool in_bounds(int x, int y, int z) {
        return x >= 0 && y >= 0 && z >= 0 &&
               x < kChunkSize && y < kChunkSize && z < kChunkSize;
    }

    // World-space coordinate of this chunk, in chunk units (multiply by
    // kChunkSize for voxel/world coordinates).
    glm::ivec3 coord{0};

private:
    std::array<VoxelType, kChunkVolume> voxels_{};
};

} // namespace lh::voxel
