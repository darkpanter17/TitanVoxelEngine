# Titan Voxel Engine

A prototype voxel engine written in Rust using `wgpu` and `winit`.

## Requirements

- Rust (Edition 2021)
- Graphics driver with Vulkan, Metal, or DirectX 12 support (used by `wgpu`)

## Running the project

```bash
cargo run
```

This will run a Lua script (`scripts/asset_downloader.lua`) to download placeholder textures, and start the voxel engine using `winit` and `wgpu`. If it cannot open a window (e.g., in headless environments), the program will gracefully exit with an error message instead of panicking.

## Controls

Use the keyboard to navigate:
- W: Move forward
- S: Move backward
- A: Move left
- D: Move right
- Space: Move up
- Shift: Move down

Mouse movement rotates the camera.

## Testing

```bash
cargo test
```
