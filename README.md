# Titan Voxel Engine

## Setup Instructions

To build the project:
```sh
cargo build
```

To run the project:
```sh
cargo run
```

To test the project:
```sh
cargo test
```

## Architecture Overview

**Titan Voxel Engine** is a voxel engine written in Rust.
It utilizes:
- `wgpu` for the graphics API (currently targeting a native window context using `winit`).
- `mlua` for scripting, allowing interaction with Lua. Currently used for asset downloading.
- `rayon` for parallel processing of terrain and meshing generation.
- `glam` for mathematics.

### Components
- **Chunk (`src/chunk.rs`)**: Voxel chunks represented as a flat array of block IDs.
- **Mesher (`src/mesher.rs`)**: Computes 2D greedy meshing and removes hidden faces. Includes logic for all 6 faces.
- **State (`src/state.rs`)**: Main application state, holds the renderer, textures, pipeline, and manages chunks and multi-threading for generation.
- **Texture (`src/texture.rs`)**: Texture arrays management. If textures can't be fetched, placeholders are created.
- **Shaders (`shaders/`)**: Uses WGSL (`voxel.wgsl`) for compiling wgpu pipelines. Includes fallback `voxel.vert` and `voxel.frag`.
- **Scripts (`scripts/`)**: Lua scripts for dynamic content configuration and asset downloads.