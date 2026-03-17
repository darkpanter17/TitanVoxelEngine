# Titan Voxel Engine

Titan Voxel Engine is a voxel engine written in Rust (2021 edition), currently at version 0.4.0. It implements the foundation of a voxel world: chunks with flat storage array, mesh generation with hidden face culling and greedy meshing, WGSL shaders for texture arrays, and a Lua scripting skeleton.

## Features
- **Data Performance**: Chunk with flat array and constant indexed access; good memory locality.
- **Mesh Architecture**: Compact vertex with layer for texture array; GPU-ready design.
- **Greedy Meshing**: Vertex reduction by fusing faces in one direction; extensible to 6 faces and 2D fusion.
- **Shaders**: WGSL, texture array, flat varying for layer; standard modern pipeline.
- **Rust**: Memory safety, zero-cost abstractions, lightweight and well-chosen dependencies (glam).
- **Extensibility**: Separated modules (chunk, mesher, texture, shader, Lua); clear space to integrate Lua and texture registry.

## Requirements
- Rust toolchain (edition 2021)
- GPU with support for `TEXTURE_BINDING_ARRAY` and `SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING`
- Internet connection (for initial texture download via Lua)

## Build Instructions
- Build the project: `cargo build`
- Run the project: `cargo run`
- Run the tests: `cargo test`

## Usage
The project creates a window using `winit` and renders the generated chunk meshes using `wgpu`. The camera can be controlled with WASD/Shift/Space and the mouse.

Assets are automatically downloaded from a remote repository via a Lua script (`scripts/asset_downloader.lua`) on the first run.
