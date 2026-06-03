#pragma once

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

namespace lh {

// Simple orbit camera for v0.1.0. It circles a target point automatically so
// the generated terrain is visible from all sides without input handling
// (a full WASD/mouse controller arrives in a later version).
class Camera {
public:
    void set_target(const glm::vec3& target) { target_ = target; }
    void set_radius(float radius) { radius_ = radius; }
    void set_height(float height) { height_ = height; }

    // Advance the orbit by `dt` seconds.
    void update(float dt) {
        angle_ += orbit_speed_ * dt;
    }

    [[nodiscard]] glm::vec3 position() const {
        return target_ + glm::vec3(std::cos(angle_) * radius_, height_,
                                   std::sin(angle_) * radius_);
    }

    [[nodiscard]] glm::mat4 view() const {
        return glm::lookAt(position(), target_, glm::vec3(0.0f, 1.0f, 0.0f));
    }

    [[nodiscard]] glm::mat4 projection(float aspect) const {
        glm::mat4 proj = glm::perspective(glm::radians(fov_), aspect, near_, far_);
        // Vulkan clip space has an inverted Y compared to OpenGL.
        proj[1][1] *= -1.0f;
        return proj;
    }

private:
    glm::vec3 target_{0.0f};
    float radius_ = 120.0f;
    float height_ = 90.0f;
    float angle_ = 0.0f;
    float orbit_speed_ = 0.25f; // radians per second
    float fov_ = 60.0f;
    float near_ = 0.1f;
    float far_ = 2000.0f;
};

} // namespace lh
