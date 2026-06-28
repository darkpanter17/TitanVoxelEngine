# Titan Voxel Engine

A Rust-based voxel engine using modern rendering capabilities with wgpu.

## Architecture

- **Language:** Rust (2021 Edition)
- **Graphics API:** wgpu (cross-platform support including Vulkan, Metal, DX12)
- **Windowing:** winit
- **Math:** glam
- **Scripting:** Lua (via mlua)
- **Multithreading:** rayon

## Features

- **Multithreaded Generation:** Terrain and chunk meshes are generated concurrently using `rayon`.
- **Greedy Meshing:** Implements full 2D greedy meshing to minimize vertex and index counts, greatly improving rendering performance.
- **Dynamic Asset Loading:** Assets are dynamically downloaded via a Lua script (`scripts/asset_downloader.lua`) on startup.
- **Texture Arrays:** Supports modern GPU rendering paths utilizing texture arrays for efficient chunk rendering.

## Getting Started

### Prerequisites

Ensure you have the Rust toolchain installed.

### Running the Engine

You can run the engine using Cargo:

```bash
cargo run
```

If you are running in a headless environment (like CI/CD), you may need to use `xvfb-run` to mock an X server:

```bash
xvfb-run -a cargo run
```

## Structure

- `src/main.rs`: Entry point and Lua script initialization.
- `src/state.rs`: Core engine state, rendering pipeline, and chunks orchestrator.
- `src/chunk.rs`: Chunk data structures and storage.
- `src/mesher.rs`: Greedy meshing algorithm converting voxel data to vertex data.
- `src/texture.rs`: Texture array loading and GPU buffer population.
- `src/camera.rs`: Basic FPS-style camera.
- `shaders/`: Contains WGSL and legacy GLSL shaders.
- `scripts/`: Lua scripts for runtime configuration and asset loading.
