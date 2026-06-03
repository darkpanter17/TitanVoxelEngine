#include <cstdlib>
#include <exception>

#include "core/Engine.hpp"
#include "core/Logger.hpp"

namespace {

// Read an optional unsigned integer environment variable.
std::uint64_t env_u64(const char* name, std::uint64_t fallback) {
    const char* value = std::getenv(name);
    if (value == nullptr) {
        return fallback;
    }
    try {
        return std::stoull(value);
    } catch (...) {
        return fallback;
    }
}

} // namespace

int main() {
    lh::EngineConfig config;
    // Allow CI / headless runs to render a fixed number of frames and exit.
    config.max_frames = env_u64("LH_MAX_FRAMES", 0);
    config.seed = static_cast<std::uint32_t>(env_u64("LH_SEED", config.seed));

    try {
        lh::Engine engine(config);
        engine.init();
        engine.run();
    } catch (const std::exception& ex) {
        lh::log_error(ex.what());
        return EXIT_FAILURE;
    }
    return EXIT_SUCCESS;
}
