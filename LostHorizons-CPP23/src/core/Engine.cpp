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
    log_info("=== Lost Horizons C++23 v0.1.0 ===");
    log_info("Initializing voxel engine...");

    window_ = std::make_unique<Window>(config_.window_width, config_.window_height,
                                       "Lost Horizons C++23 v0.1.0");
    log_info("Window created: " + std::to_string(config_.window_width) + "x" +
             std::to_string(config_.window_height));

    renderer_.init(*window_);

    generate_world();

    // Frame the terrain: orbit around the centre at a comfortable distance.
    const glm::vec3 size = world_.world_size();
    const glm::vec3 center = size * 0.5f;
    camera_.set_target({center.x, size.y * 0.5f, center.z});
    camera_.set_radius(size.x * 0.95f);
    camera_.set_height(size.y * 0.85f);

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

void Engine::update(float dt) {
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
            log_info("FPS: " + std::to_string(static_cast<int>(fps)) + " | Chunks: " +
                     std::to_string(renderer_.mesh_count()));
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
