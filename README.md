# Titan Voxel Engine

Titan Voxel Engine is a voxel-based rendering engine written in Rust. It utilizes `wgpu` for high-performance graphics rendering, `rayon` for multithreaded terrain and mesh generation, and integrates Lua scripts for extending functionality and managing assets.

## Architecture

The project consists of multiple modules aimed at building a robust voxel world:

- **Data structures (`src/chunk.rs`)**: Voxel data is efficiently stored using a flat one-dimensional array. Chunk coordinates are wrapped in a 3D grid layout.
- **Mesh generation (`src/mesher.rs`)**: Responsible for greedily evaluating solid boundaries and generating indices and vertices needed by wgpu. Out-of-bounds voxels are efficiently handled and treated as air.
- **Rendering pipeline (`src/state.rs`)**: A `wgpu` rendering layer that utilizes `WGSL` to calculate vertex positions, fragment shading, and texture array layouts. The terrain setup creates chunks and calculates sine waves to generate varied heights.
- **Textures (`src/texture.rs`)**: Handles dynamic texture fetching and fallbacks via lua integration, creating texture arrays that are bound to `wgpu` pipelines.
- **Shaders (`shaders/voxel.wgsl`)**: Written in WebGPU Shading Language (WGSL) to properly map U/V layouts, project vertices onto the window coordinate system, and provide basic alpha culling. Note: Legacy GLSL shaders exist under `src/shader_loader.rs` but are deprecated.
- **Lua Scripts (`scripts/`)**: A modular asset downloading setup built on Lua dynamically fetches fallback textures on startup using `reqwest`.

## Requirements

Before running the application, make sure to have the standard Rust toolchain installed:

- Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Vulkan compatible drivers (or alternative backend) supported by `wgpu`.

## How to build and run

Build the engine with standard cargo commands:

```bash
cargo build
```

Run the engine:

```bash
cargo run
```

Run tests to ensure system integrity:

```bash
cargo test
```

For headless environments (e.g. testing in CI containers), make sure to prefix execution commands with `xvfb-run -a` to provide a virtual frame buffer.

```bash
xvfb-run -a cargo test
xvfb-run -a cargo run
```

## Controls

- `ESC`: Exit the application.
- Movement commands depend on active integrations defined in the camera layout implementation. The window traps the mouse cursor automatically to permit standard FPS navigation mapping.
