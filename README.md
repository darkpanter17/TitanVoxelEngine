# Titan Voxel Engine

A Rust-based voxel engine utilizing modern graphics APIs through `wgpu`. This project serves as an educational testbed for voxel generation, greedy meshing, and graphics programming using Rust.

## Features

- **Chunk Data Structure**: Flat array-based `Chunk` representation (32×256×32) for highly cache-coherent access.
- **Voxel Meshing**: Greedy meshing algorithm implementations.
- **Multithreading**: `rayon` is included for scaling up procedural terrain generation and meshing over multiple chunks concurrently.
- **Texture Arrays**: Native handling of texture arrays within `WGSL` and fallback mechanisms when fetching textures dynamically.
- **Lua Scripting**: Integration with `mlua` for extending capabilities and managing external assets.

## Prerequisites

- **Rust**: Version 1.70+ recommended (using Edition 2021).
- System-specific graphics dependencies (e.g., Vulkan drivers, X11/Wayland libraries on Linux).

## Running the Engine

The engine is built using standard Cargo commands.

```bash
# Build the engine
cargo build

# Run the engine
cargo run

# Run unit tests
cargo test
```

When executing headless tests on CI or Linux without an active display server, use `xvfb-run`:

```bash
xvfb-run -a cargo run
```

## Stack Details (2025-2026)
- **Rust**: 2021 Edition
- **Graphics API**: `wgpu` v0.19 with `winit` v0.29
- **Shading**: `WGSL`
- **Math**: `glam` v0.24
- **Scripting**: Lua via `mlua` v0.9
