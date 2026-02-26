# Titan Voxel Engine v0.4.0

**Titan Voxel Engine** is a high-performance voxel rendering engine written in **Rust**. It leverages **wgpu** for modern, cross-platform graphics (Vulkan, Metal, DX12, OpenGL) and integrates **Lua** for dynamic asset management. The engine is designed to be a robust foundation for voxel-based games (like Minecraft), featuring efficient meshing algorithms, texture arrays, and a modular architecture.

---

## 🚀 Features

- **High-Performance Rendering**: Built on `wgpu` (WebGPU native), ensuring compatibility and speed across Windows, Linux, and macOS.
- **Efficient Voxel Meshing**: Implements a 6-sided greedy-style meshing algorithm with **neighbor culling** to eliminate hidden faces.
- **Texture Arrays**: Supports multiple block types (Dirt, Grass, Stone, etc.) using a single draw call via `Texture2DArray` in shaders.
- **Lua Scripting Integration**: Uses `mlua` to run Lua scripts for asset downloading and management, separating content from engine logic.
- **Camera System**: First-person "fly" camera with mouse look and keyboard movement.
- **Robust Error Handling**: Automatically falls back to placeholder textures if assets are missing or fail to download.
- **Unit Tested**: Core logic for chunks and meshing is verified with Rust unit tests.

---

## 🛠️ Technology Stack

| Component | Technology | Description |
| :--- | :--- | :--- |
| **Language** | [Rust](https://www.rust-lang.org/) (2021 Edition) | Memory-safe, high-performance systems programming. |
| **Graphics API** | [wgpu](https://wgpu.rs/) (v0.19) | Safe, portable, and idiomatic Rust wrapper over Vulkan, Metal, DX12, and GLES. |
| **Windowing** | [winit](https://github.com/rust-windowing/winit) | Cross-platform window creation and event handling. |
| **Math** | [glam](https://github.com/bitshifter/glam-rs) | Fast linear algebra library for game development (vectors, matrices). |
| **Scripting** | [mlua](https://github.com/khvzak/mlua) | High-level bindings to Lua 5.4 for scripting capabilities. |
| **Shaders** | [WGSL](https://www.w3.org/TR/WGSL/) | WebGPU Shading Language, compiled to native spir-v/msl/hlsl. |

---

## 🏗️ Architecture & How It Works

The engine is divided into several modular components located in `src/`:

### 1. Voxel Data Structure (`src/chunk.rs`)
- **Chunk**: Represents a 32x256x32 volume of voxels.
- **Storage**: Uses a flattened 1D array (`Box<[u16]>`) for cache locality and performance, avoiding the overhead of `Vec<Vec<Vec<T>>>`.
- **Indexing**: Voxels are accessed via `x + (z * 32) + (y * 32 * 32)`.
- **Block IDs**: `0` represents air (empty), while `1, 2, 3...` represent solid blocks.

### 2. Mesh Generation (`src/mesher.rs`)
- **Algorithm**: Iterates through every voxel in a chunk. For each solid voxel, it checks its 6 neighbors (Up, Down, Left, Right, Front, Back).
- **Culling**: A face is only generated if the neighbor is transparent (Air). This drastically reduces the vertex count compared to naive meshing.
- **Vertices**: Packed into a concise `Vertex` struct containing Position (`[f32; 3]`), UVs (`[f32; 2]`), and Texture Layer (`u32`).
- **Winding**: Uses Counter-Clockwise (CCW) winding order for proper face culling.

### 3. Rendering Pipeline (`src/state.rs`)
- **Initialization**: Sets up the `wgpu::Device`, `Queue`, and `Surface`.
- **Texture Arrays**: Loads multiple images (e.g., dirt.png, grass.png) into a single `Texture2DArray`. This allows the shader to select the correct texture for a face using an index (`layer`), avoiding texture switching overhead.
- **Depth Buffer**: Implements a Z-buffer (`Depth32Float`) to ensure closer objects correctly obscure further ones.
- **Pipeline**: Configured for `TriangleList` topology with `Back` face culling enabled.

### 4. Shaders (`shaders/voxel.wgsl`)
- **Vertex Shader**: Transforms local vertex positions to clip space using a View-Projection matrix (Camera). Passes the texture layer index to the fragment shader.
- **Fragment Shader**: Samples the texture array using the UV coordinates and the Layer index. Discards transparent pixels (alpha < 0.1).

### 5. Asset Management (`scripts/asset_downloader.lua`)
- **Lua Script**: A script runs at startup to download standard texture assets (CC0) from a remote repository if they don't exist locally.
- **Rust Binding**: Rust exposes a `download_file` function to Lua, bridging the gap between the script logic and the file system/network.

---

## 🎮 Controls

| Key / Input | Action |
| :--- | :--- |
| **W, A, S, D** | Move Camera (Forward, Left, Back, Right) |
| **Space** | Move Up (Fly) |
| **Shift** | Move Down (Fly) |
| **Mouse** | Look Around (Yaw/Pitch) |
| **Esc** | Close Application |

---

## 📥 Installation & Setup

### Prerequisites
- **Rust Toolchain**: Install via [rustup.rs](https://rustup.rs).
- **Git**: To clone the repository.
- **Internet Connection**: Required for the initial run to download texture assets.

### Build and Run

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/your-username/titan_voxel_engine.git
    cd titan_voxel_engine
    ```

2.  **Run in Release Mode** (Recommended for performance):
    ```bash
    cargo run --release
    ```
    *Note: The first run may take a moment to compile dependencies and download textures.*

3.  **Run Tests**:
    ```bash
    cargo test
    ```

---

## 🔮 Future Roadmap

- [ ] **Infinite Terrain**: Implement dynamic chunk loading and unloading.
- [ ] **Multithreading**: Use `rayon` to parallelize mesh generation for multiple chunks.
- [ ] **Greedy Meshing**: Optimization to merge adjacent faces of the same type into larger quads.
- [ ] **Lighting**: Implement ambient occlusion and basic sunlight propagation.
- [ ] **Physics**: basic AABB collision detection.

---

## 📄 License

This project is licensed under the **MIT License**.
