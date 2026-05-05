# Titan Voxel Engine

Titan Voxel Engine is a voxel engine written in Rust (2021 edition). It implements the foundation of a voxel world: chunks with flat storage, mesh generation with hidden face culling, a `wgpu` render pipeline using WGSL shaders for texture arrays, and Lua scripting capabilities for downloading assets.

## Features

- **Chunk Data Structure:** Flat array `Box<[u16; 32×256×32]>` indexed as `x + z*CHUNK_SIZE + y*CHUNK_SIZE*CHUNK_HEIGHT` for optimized caching and O(1) access.
- **Mesh Generation:** Generates all 6 faces for visible voxels, performing bounds checking and neighbor culling to reduce the number of vertices.
- **Graphics Pipeline:** Uses `wgpu` with WGSL shaders (`shaders/voxel.wgsl`) to render the mesh, binding a dynamically loaded 2D texture array for voxel texturing.
- **Asset Management via Lua:** Lua scripting (`mlua`) is integrated to dynamically download external textures using `reqwest` at initialization.

## Setup and Execution

To build and run the engine, you need Rust (cargo) installed on your system.
Note that the rendering engine uses `wgpu` and requires a windowing environment.

### Regular Execution

```bash
cargo run
```

### Headless Environments

In a headless environment (like a CI pipeline or a generic container), `wgpu` might fail to find an adapter to create a window surface.
You can use a virtual display server like `xvfb`:

```bash
# Install the necessary graphical drivers and Xvfb if not already installed (e.g., on Ubuntu)
sudo apt-get install -y xvfb libvulkan1 mesa-vulkan-drivers vulkan-tools

# Run with xvfb-run
xvfb-run cargo run
```

This will run the application in a virtual X11 server buffer. The engine is also designed to fail gracefully with an error log when an adapter cannot be created.

## Recent Improvements

- Fixed Lua asset downloader to use a reliable fallback image generator (`dummyimage.com`) since the external Fogleman Craft texture branch is unavailable.
- Improved mesher to generate complete 6-faced blocks.
- Added a `get_voxel_safe` mechanism for neighbor block testing, providing robust bounds-checking for mesh culling.
- Refactored rendering initialization to handle headless graphic context errors safely without panicking.
- Resolved various unused code and argument-count compiler warnings.
