#pragma once

#include <cstdint>
#include <memory>

#include <entt/entt.hpp>

#include "core/Camera.hpp"
#include "core/Window.hpp"
#include "rendering/VulkanRenderer.hpp"
#include "voxel/World.hpp"

namespace lh {

// Configuration for an engine instance.
struct EngineConfig {
    int window_width = 1280;
    int window_height = 720;
    glm::ivec3 world_dims{4, 2, 4}; // chunks along each axis
    std::uint32_t seed = 1337u;
    // If > 0, the engine renders this many frames and then exits. Used for
    // headless smoke tests / CI. 0 means run until the window is closed.
    std::uint64_t max_frames = 0;
};

// Top-level engine: owns the window, Vulkan renderer, voxel world and the ECS
// registry, and drives the main loop.
class Engine {
public:
    explicit Engine(EngineConfig config = {});
    ~Engine();

    Engine(const Engine&) = delete;
    Engine& operator=(const Engine&) = delete;

    // Initialize subsystems and generate the world. Must be called before run().
    void init();

    // Run the main loop until the window closes or max_frames is reached.
    void run();

private:
    void generate_world();
    void update(float dt);

    EngineConfig config_;
    std::unique_ptr<Window> window_;
    render::VulkanRenderer renderer_;
    voxel::World world_;
    Camera camera_;
    entt::registry registry_;

    std::uint64_t frame_count_ = 0;
};

} // namespace lh
