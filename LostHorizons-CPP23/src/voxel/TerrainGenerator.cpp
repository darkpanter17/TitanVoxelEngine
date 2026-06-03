#include "voxel/TerrainGenerator.hpp"

#include <cmath>

namespace lh::voxel {

namespace {

// Integer hash -> [0, 1) pseudo-random value. Deterministic and cheap.
float hash2(int x, int z, std::uint32_t seed) {
    std::uint32_t h = seed;
    h ^= static_cast<std::uint32_t>(x) * 0x9E3779B1u;
    h ^= static_cast<std::uint32_t>(z) * 0x85EBCA77u;
    h ^= h >> 15;
    h *= 0xD168AAADu;
    h ^= h >> 13;
    return static_cast<float>(h & 0x00FFFFFFu) / static_cast<float>(0x01000000u);
}

float smooth(float t) {
    return t * t * (3.0f - 2.0f * t); // smoothstep
}

float lerp(float a, float b, float t) {
    return a + (b - a) * t;
}

} // namespace

float TerrainGenerator::value_noise(float x, float z) const {
    const int xi = static_cast<int>(std::floor(x));
    const int zi = static_cast<int>(std::floor(z));
    const float fx = smooth(x - static_cast<float>(xi));
    const float fz = smooth(z - static_cast<float>(zi));

    const float v00 = hash2(xi, zi, seed_);
    const float v10 = hash2(xi + 1, zi, seed_);
    const float v01 = hash2(xi, zi + 1, seed_);
    const float v11 = hash2(xi + 1, zi + 1, seed_);

    return lerp(lerp(v00, v10, fx), lerp(v01, v11, fx), fz);
}

float TerrainGenerator::fractal_noise(float x, float z) const {
    float amplitude = 1.0f;
    float frequency = 1.0f;
    float sum = 0.0f;
    float norm = 0.0f;
    for (int octave = 0; octave < 4; ++octave) {
        sum += amplitude * value_noise(x * frequency, z * frequency);
        norm += amplitude;
        amplitude *= 0.5f;
        frequency *= 2.0f;
    }
    return sum / norm; // [0, 1)
}

float TerrainGenerator::height_at(int world_x, int world_z) const {
    constexpr float kScale = 0.012f;   // controls hill size
    constexpr float kBase = 18.0f;     // baseline height in voxels
    constexpr float kAmplitude = 26.0f; // peak-to-valley range
    const float n = fractal_noise(static_cast<float>(world_x) * kScale,
                                  static_cast<float>(world_z) * kScale);
    return kBase + n * kAmplitude;
}

void TerrainGenerator::generate(Chunk& chunk) const {
    const int base_x = chunk.coord.x * kChunkSize;
    const int base_y = chunk.coord.y * kChunkSize;
    const int base_z = chunk.coord.z * kChunkSize;

    constexpr int kWaterLevel = 16;
    constexpr int kSnowLevel = 38;

    for (int x = 0; x < kChunkSize; ++x) {
        for (int z = 0; z < kChunkSize; ++z) {
            const int wx = base_x + x;
            const int wz = base_z + z;
            const int surface = static_cast<int>(height_at(wx, wz));

            for (int y = 0; y < kChunkSize; ++y) {
                const int wy = base_y + y;
                VoxelType type = VoxelType::Air;

                if (wy < surface - 4) {
                    type = VoxelType::Stone;
                } else if (wy < surface - 1) {
                    type = VoxelType::Dirt;
                } else if (wy < surface) {
                    if (surface >= kSnowLevel)        type = VoxelType::Snow;
                    else if (surface <= kWaterLevel + 1) type = VoxelType::Sand;
                    else                              type = VoxelType::Grass;
                } else if (wy < kWaterLevel) {
                    type = VoxelType::Water;
                }

                chunk.set(x, y, z, type);
            }
        }
    }
}

} // namespace lh::voxel
