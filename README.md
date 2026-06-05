# Titan Voxel Engine

A multi-threaded Voxel engine built with **Rust** and **wgpu**.

## Features

- **Rendering:** Uses modern GPU APIs through `wgpu` (Vulkan, Metal, DX12, OpenGL) and custom WGSL shaders.
- **Multithreading:** Leverages `rayon` to securely generate voxel chunk terrain data and meshes concurrently.
- **Scripting:** Uses `mlua` for extending functionalities using the Lua programming language.
- **Greedy Meshing:** Implements performant culling techniques for voxel faces to reduce draw calls and memory overhead.

## Architecture

The engine is modularized primarily into the following components:

- **State (`state.rs`)**: Initializes graphics, windowing contexts, controls the render pipeline, and invokes mesh creation using thread pools.
- **Mesher (`mesher.rs`)**: Calculates required block faces and vertices based on neighbor voxel occlusion, turning chunk data into renderable meshes.
- **Chunk (`chunk.rs`)**: The fundamental chunk data structure that utilizes a flattened multi-dimensional array for cache-friendly constant time access.
- **Shaders (`shaders/voxel.wgsl`)**: The primary rendering code. Includes UV calculation, lighting (if implemented), and block layering for textures.
- **Scripts (`scripts/`)**: Uses Lua internally for asset downloading and external resource resolution.

## Build and Run

To compile and run this project, ensure you have Rust installed.

```bash
cargo run
```

To run the unit tests:

```bash
cargo test
```