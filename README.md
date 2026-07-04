# Titan Voxel Engine

A multithreaded voxel engine built in Rust using WGPU. It implements a fully functioning 2D greedy mesher across all chunk faces and provides a foundational layout for scalable, multithreaded chunk generation and rendering.

## Features

- **WGPU Pipeline:** Implemented entirely in Rust utilizing `wgpu` v0.19 and `winit` v0.29 for a cross-platform graphics API.
- **Multithreading:** Utilizes `rayon` to scale chunk data generation and mesh construction in parallel.
- **2D Greedy Meshing:** Substantially reduces quad/triangle counts across all faces (top, bottom, right, left, front, back) by analyzing and stitching contiguous, uniformly-textured blocks.
- **Lua Scripting:** Leverages `mlua` 0.9.9 to dynamically resolve and download fallback texture arrays during initialization to prevent `404 Not Found` missing asset panics.
- **Sine Wave Terrain Generation:** Simple multichunk block layout utilizing math to define terrain height and IDs.

## Requirements

- Rust 1.80+ (Edition 2021)
- Appropriate graphics hardware and Vulkan/Metal/DirectX 12 backend support depending on the OS platform.
- For headless test environments (CI), virtual framebuffer drivers might be required, e.g., `xvfb-run`.

## How to Run

1. Clone the repository.
2. Run using `cargo run`:

   ```bash
   cargo run
   ```

*(To forcefully use a specific graphics backend, pass `WGPU_BACKEND=gl` or `vulkan` environment variables).*

## Code Architecture

- `src/main.rs`: Entry point and initialization loops.
- `src/chunk.rs`: Defines 1D flat structures used for 32x256x32 voxel chunk layouts.
- `src/mesher.rs`: Handles chunk voxel conversion to 3D models via the 2D greedy mesher.
- `src/state.rs`: The bulk engine loop and graphic rendering configuration utilizing the `wgpu` bind groups.
- `scripts/asset_downloader.lua`: Lua script responsible for populating textures at runtime into `assets/textures`.
- `shaders/voxel.wgsl`: Core WebGPU Shading Language shader file.
