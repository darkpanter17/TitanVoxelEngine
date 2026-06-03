#include "voxel/World.hpp"

#include "core/Threading.hpp"

namespace lh::voxel {

std::uint64_t World::key(int cx, int cy, int cz) {
    // Pack three signed 21-bit chunk coords into one 64-bit key.
    const auto ux = static_cast<std::uint64_t>(cx + (1 << 20)) & 0x1FFFFF;
    const auto uy = static_cast<std::uint64_t>(cy + (1 << 20)) & 0x1FFFFF;
    const auto uz = static_cast<std::uint64_t>(cz + (1 << 20)) & 0x1FFFFF;
    return ux | (uy << 21) | (uz << 42);
}

void World::generate(const TerrainGenerator& generator, glm::ivec3 dims,
                     unsigned thread_count) {
    dims_ = dims;
    lookup_.clear();
    const std::size_t count = static_cast<std::size_t>(dims.x) * dims.y * dims.z;

    // Pre-size the storage and assign each chunk its coordinate up front so the
    // parallel pass only ever writes to its own, disjoint chunk. Filling the
    // lookup table serially keeps it free of data races and deterministic.
    chunks_.assign(count, Chunk{});
    lookup_.reserve(count);
    std::size_t linear = 0;
    for (int cx = 0; cx < dims.x; ++cx) {
        for (int cy = 0; cy < dims.y; ++cy) {
            for (int cz = 0; cz < dims.z; ++cz) {
                chunks_[linear].coord = {cx, cy, cz};
                lookup_[key(cx, cy, cz)] = linear;
                ++linear;
            }
        }
    }

    // Terrain generation only ever touches the chunk passed to it, so distinct
    // chunks can be generated concurrently.
    parallel_for(count, resolve_thread_count(thread_count),
                 [&](std::size_t i) { generator.generate(chunks_[i]); });
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

std::vector<ChunkMesh> World::build_meshes(unsigned thread_count) const {
    // Meshing reads neighbouring chunks (via is_solid_world) but never mutates
    // the world, so every chunk can be meshed concurrently. Each worker writes
    // to its own slot; results are compacted afterwards in chunk order so the
    // output is identical regardless of how the work was scheduled.
    std::vector<render::MeshData> meshes(chunks_.size());

    parallel_for(chunks_.size(), resolve_thread_count(thread_count), [&](std::size_t i) {
        const Chunk& chunk = chunks_[i];
        const glm::ivec3 base = chunk.coord * kChunkSize;
        const auto sampler = [&](int lx, int ly, int lz) {
            return is_solid_world(base.x + lx, base.y + ly, base.z + lz);
        };
        meshes[i] = ChunkMesher::build(chunk, sampler);
    });

    std::vector<ChunkMesh> result;
    result.reserve(chunks_.size());
    for (std::size_t i = 0; i < chunks_.size(); ++i) {
        if (!meshes[i].empty()) {
            result.push_back({chunks_[i].coord, std::move(meshes[i])});
        }
    }

    return result;
}

} // namespace lh::voxel
