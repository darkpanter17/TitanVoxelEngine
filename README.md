# Titan Voxel Engine

Titan Voxel Engine is a voxel engine written in Rust (2021 edition). It implements a base voxel world, mesh generation with neighbor face culling, texturing, and a skeleton for Lua scripting.

## Features

- **Chunk storage**: Fast flat array storage `Box<[u16; 32×256×32]>` for O(1) access.
- **Mesher**: Greedy meshing on top faces and fast back face culling on all 6 faces using multithreading (`rayon`).
- **Graphics**: `wgpu` backend with WGSL shaders using a `texture_2d_array` for voxel textures.
- **Scripting**: Lua support via `mlua` for material definition and configuration.

## Requirements

- **Rust** 1.84+ (edition 2021)
- Vulkan / DX12 / Metal capable GPU (`wgpu`)

## Building and Running

You can compile the project with standard cargo commands:

```sh
# Build the project
cargo build

# Run the engine
cargo run

# Run tests
cargo test
```

For headless environments (e.g. CI or containers), use `xvfb-run -a cargo test` or `xvfb-run -a cargo run` to automatically find a free virtual display server.

## Architecture

- `src/chunk.rs`: Chunk structure and data manipulation.
- `src/mesher.rs`: Generation of voxel meshes with geometry optimization.
- `src/state.rs`: Initialization of the `wgpu` state, window management, rendering pipeline, and chunks update.
- `src/camera.rs`: Basic camera representation for an FPS view.
- `src/texture.rs`: Texture loading and handling arrays using `wgpu`.
- `shaders/voxel.wgsl`: Main render shader supporting texture arrays.
- `scripts/`: Lua integration for material registries.
