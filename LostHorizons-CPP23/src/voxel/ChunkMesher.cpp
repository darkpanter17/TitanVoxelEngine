#include "voxel/ChunkMesher.hpp"

#include <array>
#include <cstdint>

namespace lh::voxel {

namespace {

struct Face {
    glm::ivec3 normal;
    std::array<glm::vec3, 4> corners; // CCW when viewed from outside
};

// The 6 cube faces. Corners are unit-cube offsets added to the voxel origin.
const std::array<Face, 6> kFaces = {{
    // +X
    {{1, 0, 0}, {glm::vec3{1, 0, 0}, {1, 1, 0}, {1, 1, 1}, {1, 0, 1}}},
    // -X
    {{-1, 0, 0}, {glm::vec3{0, 0, 1}, {0, 1, 1}, {0, 1, 0}, {0, 0, 0}}},
    // +Y
    {{0, 1, 0}, {glm::vec3{0, 1, 0}, {0, 1, 1}, {1, 1, 1}, {1, 1, 0}}},
    // -Y
    {{0, -1, 0}, {glm::vec3{0, 0, 1}, {0, 0, 0}, {1, 0, 0}, {1, 0, 1}}},
    // +Z
    {{0, 0, 1}, {glm::vec3{1, 0, 1}, {1, 1, 1}, {0, 1, 1}, {0, 0, 1}}},
    // -Z
    {{0, 0, -1}, {glm::vec3{0, 0, 0}, {0, 1, 0}, {1, 1, 0}, {1, 0, 0}}},
}};

} // namespace

render::MeshData ChunkMesher::build(const Chunk& chunk, const SolidSampler& solid) {
    render::MeshData mesh;
    const glm::vec3 chunk_origin = glm::vec3(chunk.coord) * static_cast<float>(kChunkSize);

    for (int x = 0; x < kChunkSize; ++x) {
        for (int y = 0; y < kChunkSize; ++y) {
            for (int z = 0; z < kChunkSize; ++z) {
                const VoxelType type = chunk.at(x, y, z);
                if (!is_solid(type)) {
                    continue;
                }
                const glm::vec3 color = voxel_color(type);
                const glm::vec3 voxel_origin = chunk_origin + glm::vec3(x, y, z);

                for (const Face& face : kFaces) {
                    // Emit the face only if the neighbour in that direction is empty.
                    if (solid(x + face.normal.x, y + face.normal.y, z + face.normal.z)) {
                        continue;
                    }

                    const auto base = static_cast<std::uint32_t>(mesh.vertices.size());
                    const glm::vec3 n = glm::vec3(face.normal);
                    for (const glm::vec3& corner : face.corners) {
                        mesh.vertices.push_back({voxel_origin + corner, n, color});
                    }
                    // Two triangles per quad (0,1,2) and (0,2,3).
                    mesh.indices.push_back(base + 0);
                    mesh.indices.push_back(base + 1);
                    mesh.indices.push_back(base + 2);
                    mesh.indices.push_back(base + 0);
                    mesh.indices.push_back(base + 2);
                    mesh.indices.push_back(base + 3);
                }
            }
        }
    }

    return mesh;
}

} // namespace lh::voxel
