# Titan Voxel Engine

A multi-threaded Voxel Engine built with Rust, `wgpu`, and Lua integration.

## Features

- **High-Performance Meshing**: Implements 2D greedy meshing to dramatically reduce the triangle count of generated voxel chunks.
- **Multithreading**: Utilizes `rayon` for parallel terrain generation and mesh building across chunks.
- **Cross-Platform Graphics**: Powered by `wgpu` with native WGSL shaders, targeting Vulkan, Metal, DX12, and OpenGL automatically.
- **Lua Scripting Integration**: Leverages `mlua` to fetch and configure block textures at runtime.
- **Efficient Texture Arrays**: Uses texture arrays (`sampler2DArray`) in `wgpu` to draw multiple chunk materials in a single draw call.

## Requirements

- **Rust Toolchain**: `rustc 1.70+` (Edition 2021)
- **Vulkan / Graphics Drivers**: Required for `wgpu` to initialize.
- Headless environments require `xvfb` to avoid display server panics during winit initialization.

## Building and Running

Compile the engine:
```bash
cargo build
```

Run the engine:
```bash
cargo run
```

Run headless tests (requires `xvfb`):
```bash
xvfb-run -a cargo test
xvfb-run -a cargo run
```

## Controls

- Move the mouse to look around (First-Person view).
- Press **Escape** to close the window gracefully.

## Project Structure

- `src/`: Core engine code (State, Camera, Chunk, Mesher, Textures).
- `shaders/`: Contains WGSL shaders (`voxel.wgsl`).
- `scripts/`: Contains `asset_downloader.lua` for fetching external textures.
- `assets/`: The destination directory for downloaded and local assets.

## License

MIT License (or your applicable license).
