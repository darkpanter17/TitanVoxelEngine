# Titan Voxel Engine

A Rust-based voxel engine utilizing modern technologies like `wgpu` for cross-platform rendering and `rayon` for parallel processing. It features a scalable, chunk-based terrain system with multi-threading support and texture mapping via Lua scripting.

## Features

- **Chunk-based Terrain Generation:** Uses a 3D grid layout (`CHUNK_SIZE` = 32, `CHUNK_HEIGHT` = 256) for generating voxels using sine waves for realistic height maps.
- **Parallel Processing:** Leverages `rayon` to perform CPU-intensive tasks like generating terrain and building voxel meshes concurrently across a 3x3 grid.
- **Optimized Voxel Meshing:** Implements efficient 6-face geometry generation with neighbor culling (hidden face removal) to significantly reduce the vertex count. Multiple chunk meshes are batched into a single buffer.
- **Modern Graphics Backend:** Powered by `wgpu` and WGSL for Vulkan/Metal/DX12 compatibility. Uses texture arrays to map voxel IDs to corresponding layers within the GPU.
- **Dynamic Asset Loading:** Uses a Lua script (`mlua`) with a built-in `reqwest` Rust fallback to download necessary CC0 texture assets asynchronously or create colored placeholders if the download fails.
- **Headless Compatibility:** Supports headless environments by properly handling adapter errors natively. Use `xvfb-run` on Linux systems if a virtual display is needed.

## Tech Stack

- **Rust:** Edition 2021
- **Graphics:** `wgpu` (v0.19), `winit` (v0.29), `pollster`
- **Math:** `glam` (v0.24)
- **Parallelism:** `rayon` (v1.8)
- **Scripting:** `mlua` (v0.9.9) for Lua 5.4 integration
- **Assets/Networking:** `image`, `reqwest`

## Getting Started

### Prerequisites
Make sure you have Rust and Cargo installed. If you are on Linux, you may need Vulkan libraries:
```bash
sudo apt-get install mesa-vulkan-drivers vulkan-utility-libraries-dev
```

### Running the Engine
Simply run:
```bash
cargo run
```
To run the project in headless mode (e.g., in a CI/CD environment):
```bash
xvfb-run -a cargo run
```

### Controls
- Look around using your mouse (FPS style).
- Press the `Escape` key to quit the application gracefully.

## Architecture

1. **State (`src/state.rs`):** Manages the core engine loop, window setup, rendering pipeline configuration, and combines multiple chunk meshes into batched GPU buffers.
2. **Chunk (`src/chunk.rs`):** Represents a 32x256x32 block of voxels. Uses a flat memory array for cache coherence (`O(1)` access).
3. **Mesher (`src/mesher.rs`):** Iterates over chunks and generates vertices/indices. Culls hidden faces between adjacent voxels.
4. **Textures (`src/texture.rs`):** Handles loading texture images as arrays. Falls back to colored `1x1` placeholders to prevent panics during missing assets.
5. **Camera (`src/camera.rs`):** FPS camera handling using `glam` for projection and view matrix updates.
6. **Assets (`scripts/asset_downloader.lua`):** Responsible for fetching textures using a Rust-exposed network function at engine launch.

## Testing

Run tests locally via:
```bash
cargo test
```
Or in headless mode:
```bash
xvfb-run -a cargo test
```
