#pragma once

#include <algorithm>
#include <cmath>

#include <glm/glm.hpp>
#include <glm/gtc/constants.hpp>
#include <glm/gtc/matrix_transform.hpp>

namespace lh {

// Camera supporting two modes:
//   * Free-fly (v0.2.0): WASD + Q/E movement with mouse look and scroll zoom,
//     with smoothed velocity. Used for interactive runs.
//   * Orbit (v0.1.0): automatically circles a target. Used for headless / CI
//     runs so a frame can be rendered without any input.
class Camera {
public:
    // Per-frame movement intent, gathered from the keyboard by the engine.
    struct MoveInput {
        bool forward = false;
        bool back = false;
        bool left = false;
        bool right = false;
        bool up = false;
        bool down = false;
        bool fast = false; // Shift: speed boost
        bool slow = false; // Ctrl: precision mode
    };

    // ---- Mode selection -----------------------------------------------------
    void set_orbit(bool enabled) { orbit_ = enabled; }
    [[nodiscard]] bool orbit() const { return orbit_; }

    // ---- Free-fly setup -----------------------------------------------------
    void set_position(const glm::vec3& p) { position_ = p; }

    // Orient the camera so it faces `target` from its current position.
    void look_at(const glm::vec3& target) {
        const glm::vec3 dir = glm::normalize(target - position_);
        pitch_ = std::asin(std::clamp(dir.y, -1.0f, 1.0f));
        yaw_ = std::atan2(dir.z, dir.x);
    }

    void set_move_input(const MoveInput& input) { move_ = input; }
    void add_look(float delta_yaw, float delta_pitch) {
        yaw_ += delta_yaw;
        pitch_ = std::clamp(pitch_ + delta_pitch, -kMaxPitch, kMaxPitch);
    }
    void add_zoom(float scroll) {
        target_fov_ = std::clamp(target_fov_ - scroll * 2.0f, 25.0f, 90.0f);
    }

    // ---- Orbit setup --------------------------------------------------------
    void set_target(const glm::vec3& target) { orbit_target_ = target; }
    void set_radius(float radius) { radius_ = radius; }
    void set_height(float height) { height_ = height; }

    // ---- Per-frame update ---------------------------------------------------
    void update(float dt) {
        // Smooth the FOV toward its target for a pleasant zoom.
        const float blend = 1.0f - std::exp(-dt * 12.0f);
        fov_ += (target_fov_ - fov_) * blend;

        if (orbit_) {
            angle_ += orbit_speed_ * dt;
            return;
        }

        // Build the desired velocity in world space from the movement intent.
        const glm::vec3 f = forward();
        const glm::vec3 r = glm::normalize(glm::cross(f, kWorldUp));
        glm::vec3 dir{0.0f};
        if (move_.forward) dir += f;
        if (move_.back) dir -= f;
        if (move_.right) dir += r;
        if (move_.left) dir -= r;
        if (move_.up) dir += kWorldUp;
        if (move_.down) dir -= kWorldUp;

        float speed = base_speed_;
        if (move_.fast) speed *= 4.0f;
        if (move_.slow) speed *= 0.25f;

        glm::vec3 target_velocity{0.0f};
        if (glm::dot(dir, dir) > 0.0f) {
            target_velocity = glm::normalize(dir) * speed;
        }

        // Exponential smoothing: snappy but jitter-free.
        const float t = 1.0f - std::exp(-dt * smoothing_);
        velocity_ += (target_velocity - velocity_) * t;
        position_ += velocity_ * dt;
    }

    // ---- Matrices -----------------------------------------------------------
    [[nodiscard]] glm::vec3 position() const {
        if (orbit_) {
            return orbit_target_ + glm::vec3(std::cos(angle_) * radius_, height_,
                                             std::sin(angle_) * radius_);
        }
        return position_;
    }

    [[nodiscard]] glm::mat4 view() const {
        if (orbit_) {
            return glm::lookAt(position(), orbit_target_, kWorldUp);
        }
        return glm::lookAt(position_, position_ + forward(), kWorldUp);
    }

    [[nodiscard]] glm::mat4 projection(float aspect) const {
        glm::mat4 proj = glm::perspective(glm::radians(fov_), aspect, near_, far_);
        // Vulkan clip space has an inverted Y compared to OpenGL.
        proj[1][1] *= -1.0f;
        return proj;
    }

    [[nodiscard]] float fov() const { return fov_; }
    [[nodiscard]] float yaw() const { return yaw_; }
    [[nodiscard]] float pitch() const { return pitch_; }

private:
    static constexpr glm::vec3 kWorldUp{0.0f, 1.0f, 0.0f};
    static constexpr float kMaxPitch = 1.55334f; // ~89 degrees

    [[nodiscard]] glm::vec3 forward() const {
        return glm::normalize(glm::vec3(std::cos(pitch_) * std::cos(yaw_),
                                        std::sin(pitch_),
                                        std::cos(pitch_) * std::sin(yaw_)));
    }

    // Free-fly state.
    glm::vec3 position_{0.0f};
    glm::vec3 velocity_{0.0f};
    float yaw_ = 0.0f;   // radians, around +Y
    float pitch_ = 0.0f; // radians
    float base_speed_ = 30.0f;  // m/s
    float smoothing_ = 12.0f;   // higher = snappier
    MoveInput move_{};

    // Orbit state.
    bool orbit_ = false;
    glm::vec3 orbit_target_{0.0f};
    float radius_ = 120.0f;
    float height_ = 90.0f;
    float angle_ = 0.0f;
    float orbit_speed_ = 0.25f;

    // Shared projection parameters.
    float fov_ = 60.0f;
    float target_fov_ = 60.0f;
    float near_ = 0.1f;
    float far_ = 3000.0f;
};

} // namespace lh
