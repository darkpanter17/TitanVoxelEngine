# Titan Voxel Engine

A Rust-based voxel engine using `wgpu` for rendering, `winit` for windowing, and `mlua` for scripting.

## Features
- **Meshing**: Generates geometry for all 6 voxel faces, with neighbor culling and proper texture mapping using texture arrays.
- **Rendering**: Uses WGSL shaders for cross-platform GPU rendering (Vulkan, Metal, D3D12, OpenGL).
- **World Generation**: Simple sine wave terrain generation across a grid of chunks, processed in parallel using `rayon`.
- **Assets**: Integrates Lua scripting for downloading default texture assets.

## Prerequisites
- Rust (edition 2021)

## Running the Engine
Execute the following command to run the project:
```sh
cargo run
```

## Running Tests
Run tests with:
```sh
cargo test
```

## Structure
- `src/`: Rust source code.
  - `chunk.rs`: Chunk structure for voxel storage.
  - `mesher.rs`: Mesh generation (geometry & greedy culling).
  - `state.rs`: The rendering pipeline and main logic loop.
  - `world.rs`: World and chunk management.
- `shaders/`: Contains the WGSL shaders.
- `scripts/`: Lua scripts for loading and downloading assets.
- `assets/`: Automatically downloaded texture files.