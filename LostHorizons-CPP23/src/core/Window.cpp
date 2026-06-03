#include "core/Window.hpp"

#include <stdexcept>

namespace lh {

Window::Window(int width, int height, std::string title) {
    if (glfwInit() != GLFW_TRUE) {
        throw std::runtime_error("Failed to initialize GLFW");
    }
    // No OpenGL context: this is a Vulkan window.
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE);

    handle_ = glfwCreateWindow(width, height, title.c_str(), nullptr, nullptr);
    if (handle_ == nullptr) {
        glfwTerminate();
        throw std::runtime_error("Failed to create GLFW window");
    }

    glfwSetWindowUserPointer(handle_, this);
    glfwSetFramebufferSizeCallback(handle_, framebuffer_resize_callback);
    glfwSetCursorPosCallback(handle_, cursor_pos_callback);
    glfwSetScrollCallback(handle_, scroll_callback);
}

void Window::set_cursor_captured(bool captured) {
    cursor_captured_ = captured;
    glfwSetInputMode(handle_, GLFW_CURSOR,
                     captured ? GLFW_CURSOR_DISABLED : GLFW_CURSOR_NORMAL);
    if (captured && glfwRawMouseMotionSupported() == GLFW_TRUE) {
        glfwSetInputMode(handle_, GLFW_RAW_MOUSE_MOTION, GLFW_TRUE);
    }
    // Force a fresh baseline so the first frame after a capture toggle does not
    // register a huge jump.
    first_mouse_ = true;
}

void Window::cursor_pos_callback(GLFWwindow* window, double x, double y) {
    auto* self = static_cast<Window*>(glfwGetWindowUserPointer(window));
    if (self == nullptr || !self->cursor_captured_) {
        return;
    }
    if (self->first_mouse_) {
        self->last_x_ = x;
        self->last_y_ = y;
        self->first_mouse_ = false;
        return;
    }
    self->mouse_dx_ += x - self->last_x_;
    self->mouse_dy_ += y - self->last_y_;
    self->last_x_ = x;
    self->last_y_ = y;
}

void Window::scroll_callback(GLFWwindow* window, double /*x_offset*/, double y_offset) {
    auto* self = static_cast<Window*>(glfwGetWindowUserPointer(window));
    if (self != nullptr) {
        self->scroll_ += y_offset;
    }
}

Window::~Window() {
    if (handle_ != nullptr) {
        glfwDestroyWindow(handle_);
    }
    glfwTerminate();
}

void Window::framebuffer_resize_callback(GLFWwindow* window, int /*width*/, int /*height*/) {
    auto* self = static_cast<Window*>(glfwGetWindowUserPointer(window));
    if (self != nullptr) {
        self->resized_ = true;
    }
}

std::vector<const char*> Window::required_instance_extensions() {
    std::uint32_t count = 0;
    const char** extensions = glfwGetRequiredInstanceExtensions(&count);
    return {extensions, extensions + count};
}

VkSurfaceKHR Window::create_surface(VkInstance instance) const {
    VkSurfaceKHR surface = VK_NULL_HANDLE;
    if (glfwCreateWindowSurface(instance, handle_, nullptr, &surface) != VK_SUCCESS) {
        return VK_NULL_HANDLE;
    }
    return surface;
}

} // namespace lh
