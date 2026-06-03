#pragma once

#include <cstdint>
#include <optional>
#include <vector>

#include <glm/glm.hpp>
#include <vulkan/vulkan.h>

#include "rendering/Frustum.hpp"
#include "rendering/Vertex.hpp"

namespace lh {
class Window;
}

namespace lh::render {

// Per-frame uniform data. std140-compatible layout (mat4 + mat4 + vec4).
struct UniformBufferObject {
    glm::mat4 view;
    glm::mat4 proj;
    glm::vec4 light_dir; // xyz = direction, w unused
};

// A GPU-resident mesh (device-local vertex + index buffers) plus the world-space
// bounding box used for frustum culling.
struct GpuMesh {
    VkBuffer vertex_buffer = VK_NULL_HANDLE;
    VkDeviceMemory vertex_memory = VK_NULL_HANDLE;
    VkBuffer index_buffer = VK_NULL_HANDLE;
    VkDeviceMemory index_memory = VK_NULL_HANDLE;
    std::uint32_t index_count = 0;
    glm::vec3 aabb_min{0.0f};
    glm::vec3 aabb_max{0.0f};
};

// Core Vulkan renderer: owns the instance, device, swapchain, pipeline and
// command recording. Split across VulkanRenderer_*.cpp by responsibility,
// matching the engine's documented file layout.
class VulkanRenderer {
public:
    static constexpr int kMaxFramesInFlight = 2;

    void init(Window& window);
    void cleanup();

    // Upload a CPU mesh to device-local memory and return its handle.
    [[nodiscard]] std::uint32_t upload_mesh(const MeshData& mesh);

    // Render one frame, drawing every uploaded mesh with the given camera.
    void draw_frame(const glm::mat4& view, const glm::mat4& proj);

    void wait_idle() const;

    [[nodiscard]] std::uint32_t mesh_count() const {
        return static_cast<std::uint32_t>(meshes_.size());
    }

    // Number of meshes that passed frustum culling in the last frame.
    [[nodiscard]] std::uint32_t visible_count() const { return visible_count_; }

private:
    struct QueueFamilyIndices {
        std::optional<std::uint32_t> graphics;
        std::optional<std::uint32_t> present;
        [[nodiscard]] bool complete() const {
            return graphics.has_value() && present.has_value();
        }
    };

    struct SwapchainSupport {
        VkSurfaceCapabilitiesKHR capabilities{};
        std::vector<VkSurfaceFormatKHR> formats;
        std::vector<VkPresentModeKHR> present_modes;
    };

    // --- VulkanRenderer_Init.cpp ---
    void create_instance();
    void setup_debug_messenger();
    void pick_physical_device();
    void create_logical_device();
    void create_command_pool();
    void create_uniform_buffers();
    void create_descriptor_pool();
    void create_descriptor_sets();
    void create_command_buffers();
    void create_sync_objects();
    [[nodiscard]] bool is_device_suitable(VkPhysicalDevice device) const;
    [[nodiscard]] QueueFamilyIndices find_queue_families(VkPhysicalDevice device) const;
    [[nodiscard]] std::uint32_t find_memory_type(std::uint32_t type_filter,
                                                 VkMemoryPropertyFlags properties) const;
    void create_buffer(VkDeviceSize size, VkBufferUsageFlags usage,
                       VkMemoryPropertyFlags properties, VkBuffer& buffer,
                       VkDeviceMemory& memory) const;
    void copy_buffer(VkBuffer src, VkBuffer dst, VkDeviceSize size) const;

    // --- VulkanRenderer_Swapchain.cpp ---
    void create_swapchain();
    void create_image_views();
    void create_depth_resources();
    void create_framebuffers();
    void recreate_swapchain();
    void cleanup_swapchain();
    [[nodiscard]] SwapchainSupport query_swapchain_support(VkPhysicalDevice device) const;
    [[nodiscard]] VkFormat find_depth_format() const;

    // --- VulkanRenderer_Pipeline.cpp ---
    void create_render_pass();
    void create_descriptor_set_layout();
    void create_graphics_pipeline();
    [[nodiscard]] VkShaderModule create_shader_module(const std::vector<char>& code) const;

    // --- VulkanRenderer_Render.cpp ---
    void record_command_buffer(VkCommandBuffer cmd, std::uint32_t image_index);
    void update_uniform_buffer(std::uint32_t current_image, const glm::mat4& view,
                               const glm::mat4& proj);

    // Owned externally.
    Window* window_ = nullptr;

    // Core objects.
    VkInstance instance_ = VK_NULL_HANDLE;
    VkDebugUtilsMessengerEXT debug_messenger_ = VK_NULL_HANDLE;
    VkSurfaceKHR surface_ = VK_NULL_HANDLE;
    VkPhysicalDevice physical_device_ = VK_NULL_HANDLE;
    VkDevice device_ = VK_NULL_HANDLE;
    VkQueue graphics_queue_ = VK_NULL_HANDLE;
    VkQueue present_queue_ = VK_NULL_HANDLE;
    std::uint32_t graphics_family_ = 0;
    std::uint32_t present_family_ = 0;

    // Swapchain.
    VkSwapchainKHR swapchain_ = VK_NULL_HANDLE;
    std::vector<VkImage> swapchain_images_;
    std::vector<VkImageView> swapchain_image_views_;
    std::vector<VkFramebuffer> swapchain_framebuffers_;
    VkFormat swapchain_format_ = VK_FORMAT_UNDEFINED;
    VkExtent2D swapchain_extent_{};

    // Depth.
    VkImage depth_image_ = VK_NULL_HANDLE;
    VkDeviceMemory depth_memory_ = VK_NULL_HANDLE;
    VkImageView depth_view_ = VK_NULL_HANDLE;
    VkFormat depth_format_ = VK_FORMAT_UNDEFINED;

    // Pipeline.
    VkRenderPass render_pass_ = VK_NULL_HANDLE;
    VkDescriptorSetLayout descriptor_set_layout_ = VK_NULL_HANDLE;
    VkPipelineLayout pipeline_layout_ = VK_NULL_HANDLE;
    VkPipeline graphics_pipeline_ = VK_NULL_HANDLE;

    // Commands & sync.
    VkCommandPool command_pool_ = VK_NULL_HANDLE;
    std::vector<VkCommandBuffer> command_buffers_;
    std::vector<VkSemaphore> image_available_;
    std::vector<VkSemaphore> render_finished_;
    std::vector<VkFence> in_flight_;
    std::uint32_t current_frame_ = 0;

    // Uniforms.
    std::vector<VkBuffer> uniform_buffers_;
    std::vector<VkDeviceMemory> uniform_memory_;
    std::vector<void*> uniform_mapped_;
    VkDescriptorPool descriptor_pool_ = VK_NULL_HANDLE;
    std::vector<VkDescriptorSet> descriptor_sets_;

    // Meshes.
    std::vector<GpuMesh> meshes_;
    Frustum frustum_;
    std::uint32_t visible_count_ = 0;

    bool validation_enabled_ = false;
};

} // namespace lh::render
