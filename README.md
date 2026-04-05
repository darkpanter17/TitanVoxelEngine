# Titan Voxel Engine

Titan Voxel Engine is a high-performance voxel engine written in Rust using WGPU. It is designed to be multi-platform, extensible, and efficient.

## Features

- **6-Face Greedy Mesher:** Generates optimized mesh geometry dynamically handling occlusions through accurate voxel neighbor checking for all 6 voxel faces.
- **WGPU Render Pipeline:** Utilizes WGSL and WGPU to deliver high-performance GPU-accelerated rendering capable of multi-platform execution including Vulkan, Metal, D3D12, and WebGPU.
- **Dynamic Terrain Generation:** A parallelized procedural terrain generation system leveraging sine waves across an arbitrary grid of chunks. Terrain block IDs are natively mapped to GPU texture array layers.
- **Lua Integration via MLUA:** Designed for extensive modding capabilities and dynamic configurations. `mlua` fetches assets via Lua scripts gracefully.
- **Headless Graceful Degradation:** Provides strong error handling with window initializations enabling the application to run correctly in constrained environments or CI tools.
- **Parallel processing:** Leverages `rayon` to parse and build chunks data using highly scalable multithreading architecture.

## Getting Started

### Prerequisites

You need the stable Rust toolchain (edition 2021) installed. We recommend compiling and running in release mode for best performance.

```bash
cargo run --release
```

### Dependencies
- `glam` for 3D Math logic.
- `wgpu` and `winit` for graphics context and window event loop management.
- `mlua` for executing mod scripts safely.
- `reqwest` and `image` to fetch texture map fallbacks during initialization.
- `rayon` for concurrent chunk parallelization.

## Architecture Guidelines

- **Assets:** Ensure scripts and textues are accessible locally under the `assets/` and `scripts/` directories relative to the execution context.
- **Shaders:** Modern WGPU relies primarily on WGSL, and this engine processes shaders located in `shaders/voxel.wgsl`.

For further historical insights or plans see the memory traces or `STACK_2025_2026.md`.
