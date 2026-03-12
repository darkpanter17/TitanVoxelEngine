# Titan Voxel Engine

A Rust-based voxel engine utilizing modern graphics APIs.

## Overview

Titan Voxel Engine is built on the 2021 Rust edition and leverages the `wgpu` ecosystem for robust, cross-platform graphics rendering, while using `winit` for windowing events. The engine currently implements parallelized greedy meshing, multi-chunk terrain generation, neighbor face culling, and features an expandable architecture allowing for Lua script integration.

## Requirements

To run this project, you will need:

-   Rust (stable channel, 1.80+)
-   A system capable of supporting `wgpu` graphics pipelines (Vulkan, Metal, D3D12, or OpenGL).

## Running

1. Clone the repository.
2. Build and run via Cargo:

```bash
cargo run
```

This command will compile the engine and fetch test textures from the internet via the included Lua script. If downloads fail, the engine defaults to functional colored placeholder textures.

## Features
-   Parallelized Mesh Generation across multiple chunks.
-   Neighbor Face Culling for optimized geometry.
-   Flat memory layout for Chunks to enhance data caching efficiency.
-   Dynamic Texture Array processing using WGSL and `wgpu`.

## Controls
-   **W/A/S/D** - Move around the map.
-   **Space** - Move Up.
-   **Shift** - Move Down.
-   **Mouse** - Look around.
