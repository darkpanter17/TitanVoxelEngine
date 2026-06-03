#include "core/Engine.hpp"

#include <chrono>
#include <string>

#include "core/Logger.hpp"
#include "ecs/Components.hpp"
#include "voxel/TerrainGenerator.hpp"

namespace lh {

Engine::Engine(EngineConfig config) : config_(config) {}

Engine::~Engine() {
    renderer_.cleanup();
}

void Engine::init() {
    log_info("=== Lost Horizons C++23 v0.2.0 ===");
    log_info("Initializing voxel engine...");

    // Headless / CI runs (LH_MAX_FRAMES > 0) use the auto-orbit camera so a
    // frame can render without any input; interactive runs use the free-fly
    // camera driven by the keyboard and mouse.
    interactive_ = (config_.max_frames == 0);

    window_ = std::make_unique<Window>(config_.window_width, config_.window_height,
                                       "Lost Horizons C++23 v0.2.0");
    log_info("Window created: " + std::to_string(config_.window_width) + "x" +
             std::to_string(config_.window_height));

    renderer_.init(*window_);

    generate_world();

    const glm::vec3 size = world_.world_size();
    const glm::vec3 center = size * 0.5f;
    if (interactive_) {
        // Free-fly: start above and behind the terrain, looking at its centre.
        camera_.set_orbit(false);
        camera_.set_position({center.x, size.y * 1.15f, center.z + size.z * 0.9f});
        camera_.look_at({center.x, size.y * 0.35f, center.z});
        window_->set_cursor_captured(true);
        log_info("Controls: WASD move, Q/E down/up, Shift fast, Ctrl slow, "
                 "mouse look, scroll zoom, ESC release cursor.");
    } else {
        // Orbit around the centre at a comfortable distance.
        camera_.set_orbit(true);
        camera_.set_target({center.x, size.y * 0.5f, center.z});
        camera_.set_radius(size.x * 0.95f);
        camera_.set_height(size.y * 0.85f);
    }

    log_info("Initialization complete!");
}

void Engine::generate_world() {
    log_info("Generating voxel world...");
    const voxel::TerrainGenerator generator(config_.seed);
    world_.generate(generator, config_.world_dims);
    log_info("Generated " + std::to_string(world_.chunks().size()) + " chunks (seed " +
             std::to_string(generator.seed()) + ")");

    log_info("Meshing chunks and uploading to GPU...");
    std::vector<voxel::ChunkMesh> meshes = world_.build_meshes();

    std::uint64_t total_triangles = 0;
    for (const voxel::ChunkMesh& chunk_mesh : meshes) {
        const std::uint32_t handle = renderer_.upload_mesh(chunk_mesh.mesh);
        total_triangles += chunk_mesh.mesh.indices.size() / 3;

        // Register an ECS entity per chunk (transform + mesh + chunk tag).
        const entt::entity entity = registry_.create();
        registry_.emplace<ecs::ChunkComponent>(entity, chunk_mesh.coord);
        registry_.emplace<ecs::TransformComponent>(entity);
        registry_.emplace<ecs::MeshComponent>(
            entity, handle, static_cast<std::uint32_t>(chunk_mesh.mesh.indices.size()));
    }

    log_info("Uploaded " + std::to_string(meshes.size()) + " chunk meshes (" +
             std::to_string(total_triangles) + " triangles)");
}

void Engine::process_input(float dt) {
    (void)dt;
    if (!interactive_) {
        return;
    }

    // ESC toggles cursor capture (edge-triggered).
    const bool esc_down = window_->is_key_down(GLFW_KEY_ESCAPE);
    if (esc_down && !esc_was_down_) {
        window_->set_cursor_captured(!window_->cursor_captured());
    }
    esc_was_down_ = esc_down;

    Camera::MoveInput in{};
    in.forward = window_->is_key_down(GLFW_KEY_W);
    in.back = window_->is_key_down(GLFW_KEY_S);
    in.left = window_->is_key_down(GLFW_KEY_A);
    in.right = window_->is_key_down(GLFW_KEY_D);
    in.up = window_->is_key_down(GLFW_KEY_E);
    in.down = window_->is_key_down(GLFW_KEY_Q);
    in.fast = window_->is_key_down(GLFW_KEY_LEFT_SHIFT) ||
              window_->is_key_down(GLFW_KEY_RIGHT_SHIFT);
    in.slow = window_->is_key_down(GLFW_KEY_LEFT_CONTROL) ||
              window_->is_key_down(GLFW_KEY_RIGHT_CONTROL);
    camera_.set_move_input(in);

    double dx = 0.0;
    double dy = 0.0;
    window_->take_mouse_delta(dx, dy);
    if (window_->cursor_captured()) {
        camera_.add_look(static_cast<float>(dx) * mouse_sensitivity_,
                         -static_cast<float>(dy) * mouse_sensitivity_);
    }
    camera_.add_zoom(static_cast<float>(window_->take_scroll()));
}

void Engine::update(float dt) {
    process_input(dt);
    camera_.update(dt);
}

void Engine::run() {
    using clock = std::chrono::high_resolution_clock;
    auto last_time = clock::now();
    auto last_report = last_time;
    int frames_since_report = 0;

    log_info("Entering main loop...");

    while (!window_->should_close()) {
        window_->poll_events();

        const auto now = clock::now();
        const float dt = std::chrono::duration<float>(now - last_time).count();
        last_time = now;

        update(dt);

        int width = 0;
        int height = 0;
        window_->framebuffer_size(width, height);
        const float aspect =
            (height > 0) ? static_cast<float>(width) / static_cast<float>(height) : 1.0f;

        renderer_.draw_frame(camera_.view(), camera_.projection(aspect));

        ++frame_count_;
        ++frames_since_report;

        const float since_report =
            std::chrono::duration<float>(now - last_report).count();
        if (since_report >= 1.0f) {
            const float fps = static_cast<float>(frames_since_report) / since_report;
            log_info("FPS: " + std::to_string(static_cast<int>(fps)) +
                     " | Chunks: " + std::to_string(renderer_.visible_count()) + "/" +
                     std::to_string(renderer_.mesh_count()) + " visible");
            last_report = now;
            frames_since_report = 0;
        }

        if (config_.max_frames > 0 && frame_count_ >= config_.max_frames) {
            log_info("Reached max_frames (" + std::to_string(config_.max_frames) +
                     "), exiting.");
            break;
        }
    }

    renderer_.wait_idle();
    log_info("Shutdown complete. Rendered " + std::to_string(frame_count_) + " frames.");
}

} // namespace lh
