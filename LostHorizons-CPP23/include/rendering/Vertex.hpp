#pragma once

#include <array>
#include <cstdint>
#include <vector>
#include <glm/glm.hpp>
#include <vulkan/vulkan.h>

namespace lh::render {

// Interleaved vertex format produced by the voxel mesher and consumed by the
// graphics pipeline: position, normal and a baked albedo color.
struct Vertex {
    glm::vec3 position;
    glm::vec3 normal;
    glm::vec3 color;

    [[nodiscard]] static VkVertexInputBindingDescription binding_description() {
        VkVertexInputBindingDescription binding{};
        binding.binding = 0;
        binding.stride = sizeof(Vertex);
        binding.inputRate = VK_VERTEX_INPUT_RATE_VERTEX;
        return binding;
    }

    [[nodiscard]] static std::array<VkVertexInputAttributeDescription, 3>
    attribute_descriptions() {
        std::array<VkVertexInputAttributeDescription, 3> attrs{};
        attrs[0] = {0, 0, VK_FORMAT_R32G32B32_SFLOAT, offsetof(Vertex, position)};
        attrs[1] = {1, 0, VK_FORMAT_R32G32B32_SFLOAT, offsetof(Vertex, normal)};
        attrs[2] = {2, 0, VK_FORMAT_R32G32B32_SFLOAT, offsetof(Vertex, color)};
        return attrs;
    }
};

// CPU-side mesh produced by the mesher before GPU upload.
struct MeshData {
    std::vector<Vertex> vertices;
    std::vector<std::uint32_t> indices;

    [[nodiscard]] bool empty() const { return indices.empty(); }
};

} // namespace lh::render
