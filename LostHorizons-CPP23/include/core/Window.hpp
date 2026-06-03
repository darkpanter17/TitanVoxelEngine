#pragma once

#include <cstdint>
#include <string>
#include <vector>

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

namespace lh {

// RAII wrapper around a GLFW window configured for Vulkan rendering.
class Window {
public:
    Window(int width, int height, std::string title);
    ~Window();

    Window(const Window&) = delete;
    Window& operator=(const Window&) = delete;

    [[nodiscard]] bool should_close() const { return glfwWindowShouldClose(handle_); }
    void poll_events() const { glfwPollEvents(); }

    // Vulkan instance extensions required by GLFW for surface creation.
    [[nodiscard]] static std::vector<const char*> required_instance_extensions();

    // Create a Vulkan surface for this window. Returns VK_NULL_HANDLE on failure.
    [[nodiscard]] VkSurfaceKHR create_surface(VkInstance instance) const;

    // Current framebuffer size in pixels (0x0 when minimized).
    void framebuffer_size(int& width, int& height) const {
        glfwGetFramebufferSize(handle_, &width, &height);
    }

    [[nodiscard]] bool was_resized() const { return resized_; }
    void reset_resized_flag() { resized_ = false; }

    [[nodiscard]] GLFWwindow* handle() const { return handle_; }

private:
    static void framebuffer_resize_callback(GLFWwindow* window, int width, int height);

    GLFWwindow* handle_ = nullptr;
    bool resized_ = false;
};

} // namespace lh
