# Titan Voxel Engine

A Rust-based voxel engine using `wgpu` for rendering and `mlua` for scripting assets.

## Features

- **Voxel Rendering**: Efficient 6-sided meshing with neighbor culling.
- **Texture Arrays**: Supports multiple block types using texture arrays.
- **WGPU**: Uses WebGPU-native API for cross-platform rendering (Vulkan, Metal, DX12, OpenGL).
- **Lua Integration**: Downloads texture assets via Lua script.
- **Camera**: First-person fly camera.

## Requirements

- Rust (latest stable)
- Internet connection (for initial asset download)

## Installation

1.  Clone the repository.
2.  Run `cargo run --release`.

The engine will automatically download texture assets to `assets/textures/` on the first run.

## Controls

- **W, A, S, D**: Move camera.
- **Space**: Move Up.
- **Shift**: Move Down.
- **Mouse**: Look around.
- **Esc**: Close window.

## Development

- `cargo test`: Run unit tests.
- `cargo check`: Check for compilation errors.

## Project Structure

- `src/`: Source code.
  - `chunk.rs`: Voxel data structure.
  - `mesher.rs`: Mesh generation logic.
  - `state.rs`: WGPU rendering state.
  - `texture.rs`: Texture loading.
- `shaders/`: WGSL shaders.
- `scripts/`: Lua scripts for asset management.
- `assets/`: Textures (downloaded automatically).

## License

MIT
