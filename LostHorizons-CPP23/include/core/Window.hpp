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
    void request_close() { glfwSetWindowShouldClose(handle_, GLFW_TRUE); }

    // ---- Input --------------------------------------------------------------
    [[nodiscard]] bool is_key_down(int key) const {
        return glfwGetKey(handle_, key) == GLFW_PRESS;
    }

    // Lock the cursor to the window for mouse-look (or release it).
    void set_cursor_captured(bool captured);
    [[nodiscard]] bool cursor_captured() const { return cursor_captured_; }

    // Mouse movement accumulated since the last call, then reset to zero.
    void take_mouse_delta(double& dx, double& dy) {
        dx = mouse_dx_;
        dy = mouse_dy_;
        mouse_dx_ = 0.0;
        mouse_dy_ = 0.0;
    }

    // Scroll accumulated since the last call, then reset to zero.
    [[nodiscard]] double take_scroll() {
        const double s = scroll_;
        scroll_ = 0.0;
        return s;
    }

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
    static void cursor_pos_callback(GLFWwindow* window, double x, double y);
    static void scroll_callback(GLFWwindow* window, double x_offset, double y_offset);

    GLFWwindow* handle_ = nullptr;
    bool resized_ = false;

    bool cursor_captured_ = false;
    bool first_mouse_ = true;
    double last_x_ = 0.0;
    double last_y_ = 0.0;
    double mouse_dx_ = 0.0;
    double mouse_dy_ = 0.0;
    double scroll_ = 0.0;
};

} // namespace lh
