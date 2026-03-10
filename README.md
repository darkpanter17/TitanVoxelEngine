# Titan Voxel Engine

Titan Voxel Engine is a high-performance voxel engine built in Rust using WebGPU for rendering and `winit` for window management. It generates voxel terrain, performs back-face culling, constructs meshes efficiently, and renders them.

## Requirements

Ensure you have the following installed to build and run the engine:

- **Rust toolchain:** Edition 2021 minimum (compatible up to 1.84.x)
- C/C++ compiler for dependencies, if needed (e.g. clang)
- CMake
- A GPU that supports Vulkan, DX12, Metal, or OpenGL 4.5+

## Setup & Build

1. Clone the repository and `cd` into it:
   ```bash
   git clone <repo_url> titan_voxel_engine
   cd titan_voxel_engine
   ```

2. Build the project
   ```bash
   cargo build
   ```

3. Run the project
   ```bash
   cargo run
   ```

4. Run the test suite
   ```bash
   cargo test
   ```

## Controls

The camera implements a basic first-person view:
- **W / A / S / D**: Move Forward / Left / Backward / Right
- **Space**: Move Up
- **Left Shift**: Move Down
- **Mouse Movement**: Rotate the camera (pitch and yaw are restricted vertically)

## Architecture

- `src/chunk.rs`: The chunk component holds chunk structures as a flat, single-dimensional array, allowing rapid retrieval of voxel data.
- `src/mesher.rs`: Builds geometric representations of the chunk data to stream to the GPU. Only renders exposed blocks.
- `src/state.rs`: Stores `wgpu` state initializing `wgpu::Instance`, creating pipelines from WGSL shaders, binding groups, buffers and texturing handling.
- `src/camera.rs`: Handles viewport view and projection matrix generation.
- `shaders/voxel.wgsl`: Contains standard WGSL for geometry rendering with basic diffuse texture implementation using 2D texture arrays.

## Lua Scripting
Currently, basic Lua scripts via `mlua` are partially integrated for asset downloading (`scripts/asset_downloader.lua`). Extended runtime capabilities to handle block materials and data logic configurations via `.lua` files might be integrated down the road.
