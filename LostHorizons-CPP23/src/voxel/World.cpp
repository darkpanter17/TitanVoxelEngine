#include "voxel/World.hpp"

namespace lh::voxel {

std::uint64_t World::key(int cx, int cy, int cz) {
    // Pack three signed 21-bit chunk coords into one 64-bit key.
    const auto ux = static_cast<std::uint64_t>(cx + (1 << 20)) & 0x1FFFFF;
    const auto uy = static_cast<std::uint64_t>(cy + (1 << 20)) & 0x1FFFFF;
    const auto uz = static_cast<std::uint64_t>(cz + (1 << 20)) & 0x1FFFFF;
    return ux | (uy << 21) | (uz << 42);
}

void World::generate(const TerrainGenerator& generator, glm::ivec3 dims) {
    dims_ = dims;
    chunks_.clear();
    lookup_.clear();
    chunks_.reserve(static_cast<std::size_t>(dims.x) * dims.y * dims.z);

    for (int cx = 0; cx < dims.x; ++cx) {
        for (int cy = 0; cy < dims.y; ++cy) {
            for (int cz = 0; cz < dims.z; ++cz) {
                Chunk chunk;
                chunk.coord = {cx, cy, cz};
                generator.generate(chunk);
                lookup_[key(cx, cy, cz)] = chunks_.size();
                chunks_.push_back(std::move(chunk));
            }
        }
    }
}

const Chunk* World::chunk_at(int cx, int cy, int cz) const {
    const auto it = lookup_.find(key(cx, cy, cz));
    if (it == lookup_.end()) {
        return nullptr;
    }
    return &chunks_[it->second];
}

bool World::is_solid_world(int wx, int wy, int wz) const {
    if (wx < 0 || wy < 0 || wz < 0) {
        return false;
    }
    const int cx = wx / kChunkSize;
    const int cy = wy / kChunkSize;
    const int cz = wz / kChunkSize;
    const Chunk* chunk = chunk_at(cx, cy, cz);
    if (chunk == nullptr) {
        return false;
    }
    return is_solid(chunk->at(wx - cx * kChunkSize,
                              wy - cy * kChunkSize,
                              wz - cz * kChunkSize));
}

std::vector<ChunkMesh> World::build_meshes() const {
    std::vector<ChunkMesh> result;
    result.reserve(chunks_.size());

    for (const Chunk& chunk : chunks_) {
        const glm::ivec3 base = chunk.coord * kChunkSize;
        const auto sampler = [&](int lx, int ly, int lz) {
            return is_solid_world(base.x + lx, base.y + ly, base.z + lz);
        };
        render::MeshData mesh = ChunkMesher::build(chunk, sampler);
        if (!mesh.empty()) {
            result.push_back({chunk.coord, std::move(mesh)});
        }
    }

    return result;
}

} // namespace lh::voxel
