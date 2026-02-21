# Titan Voxel Engine

A simple voxel engine written in Rust using `wgpu` for rendering.

## Features

-   **Voxel Rendering**: Uses `wgpu` for cross-platform rendering (Metal, Vulkan, DX12, OpenGL).
-   **Chunk System**: Stores voxel data in a flat array for performance.
-   **Meshing**: Standard culling algorithm to generate geometry only for exposed faces.
-   **Texture Arrays**: Uses texture arrays for efficient block texturing.
-   **Terrain Generation**: Generates a simple sine-wave based terrain with Grass, Dirt, and Stone.
-   **Lua Scripting**: Includes basic asset downloading capabilities via Lua (see `scripts/asset_downloader.lua`).

## Prerequisites

-   Rust (stable)
-   Cargo

## How to Run

1.  Clone the repository.
2.  Run the following command in the project root:

```bash
cargo run
```

This will compile the project, download necessary textures (using the embedded Lua script), and launch the window.

## Controls

-   **W / A / S / D**: Move Forward / Left / Backward / Right
-   **Space**: Move Up
-   **Left Shift**: Move Down
-   **Mouse**: Look around

## Project Structure

-   `src/`: Rust source code.
    -   `main.rs`: Entry point.
    -   `state.rs`: WGPU state and rendering loop.
    -   `mesher.rs`: Mesh generation logic.
    -   `chunk.rs`: Voxel data structure.
    -   `texture.rs`: Texture loading and management.
    -   `camera.rs`: Camera logic.
-   `shaders/`: WGSL shaders.
-   `scripts/`: Lua scripts.
-   `assets/`: Downloaded assets (textures).

## License

MIT
