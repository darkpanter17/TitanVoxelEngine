#pragma once

#include <cstdint>
#include <glm/glm.hpp>

namespace lh::ecs {

// World-space transform of an entity. v0.1.0 chunks use an identity transform
// (mesh vertices are baked in world space) but the component is kept so future
// versions can move/instantiate entities.
struct TransformComponent {
    glm::vec3 position{0.0f};
    glm::vec3 rotation{0.0f};
    glm::vec3 scale{1.0f};
};

// Links an entity to a GPU mesh owned by the renderer.
struct MeshComponent {
    std::uint32_t gpu_handle{0};
    std::uint32_t index_count{0};
};

// Marks an entity as a voxel chunk and records its grid coordinate.
struct ChunkComponent {
    glm::ivec3 coord{0};
};

} // namespace lh::ecs
