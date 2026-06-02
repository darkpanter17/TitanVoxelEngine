# Titan Voxel Engine

Titan Voxel Engine is a voxel engine written in Rust using `wgpu` and `winit`.

## Features
- Hardware-accelerated graphics via `wgpu`.
- Voxel terrain generation using `rayon` for parallelization.
- Support for generating chunk meshes utilizing greedy meshing.
- Texture mapping for multiple voxel types via texture arrays.
- Simple FPS-style camera.
- Scripts using Lua to download assets.

## Requirements
- Rust (Edition 2021)
- X11/Wayland dependencies if running on Linux (e.g., `mesa-vulkan-drivers` and `vulkan-utility-libraries-dev` may be required for `wgpu` to initialize on headless environments)

## Usage

Run the project with:

```sh
cargo run
```

Tests can be run with:

```sh
cargo test
```
