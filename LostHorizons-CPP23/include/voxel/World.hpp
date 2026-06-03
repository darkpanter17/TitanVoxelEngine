#pragma once

#include <cstdint>
#include <unordered_map>
#include <vector>
#include <glm/glm.hpp>

#include "rendering/Vertex.hpp"
#include "voxel/ChunkMesher.hpp"
#include "voxel/TerrainGenerator.hpp"
#include "voxel/VoxelData.hpp"

namespace lh::voxel {

// A built chunk together with its generated mesh, ready for GPU upload.
struct ChunkMesh {
    glm::ivec3 coord{0};
    render::MeshData mesh;
};

// Holds all generated chunks and produces seamless meshes by sampling across
// chunk boundaries.
class World {
public:
    // Generate a `dims` grid of chunks (in chunk units) using `generator`.
    void generate(const TerrainGenerator& generator, glm::ivec3 dims);

    // Build a face-culled mesh for every non-empty chunk.
    [[nodiscard]] std::vector<ChunkMesh> build_meshes() const;

    // Solid test in world voxel coordinates (out-of-world is treated as air).
    [[nodiscard]] bool is_solid_world(int wx, int wy, int wz) const;

    [[nodiscard]] const std::vector<Chunk>& chunks() const { return chunks_; }
    [[nodiscard]] glm::ivec3 dims() const { return dims_; }

    // Bounding box of the generated world in world voxels.
    [[nodiscard]] glm::vec3 world_size() const {
        return glm::vec3(dims_) * static_cast<float>(kChunkSize);
    }

private:
    [[nodiscard]] static std::uint64_t key(int cx, int cy, int cz);
    [[nodiscard]] const Chunk* chunk_at(int cx, int cy, int cz) const;

    std::vector<Chunk> chunks_;
    std::unordered_map<std::uint64_t, std::size_t> lookup_;
    glm::ivec3 dims_{0};
};

} // namespace lh::voxel
