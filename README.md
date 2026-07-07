# Titan Voxel Engine

A Rust-based voxel engine utilizing modern tooling such as `wgpu` (v0.19) for cross-platform rendering, `rayon` for parallel processing, and `mlua` for extensibility.

## Architecture

* **Renderer**: Native WGSL shaders (`shaders/voxel.wgsl`) running via the `wgpu` pipeline (`src/state.rs`), rendering texture arrays natively onto generated voxel meshes.
* **Mesher**: The geometry is constructed using 2D greedy meshing (`src/mesher.rs`) with neighbor-culling for all 6 faces, heavily reducing the vertex count. Face orientations and extents map strictly based on the face direction to prevent distortion. Meshing utilizes multiple CPU threads (`rayon` parallel iterators).
* **Chunk Storage**: Flat 1D arrays (`[u16; 32 * 256 * 32]`) inside `src/chunk.rs` ensure fast cache-aligned `O(1)` memory access.
* **Assets**: A Lua script (`scripts/asset_downloader.lua`) handles downloading individual assets on launch to construct texture arrays directly in memory, using placeholder fallbacks if external sources are unavailable.
* **Camera**: A standard First-Person perspective camera controlled through standard WASD and mouse input (`src/camera.rs`).

## Running the Project

```bash
cargo run
```
Press `ESC` to exit the application.
