# Titan Voxel Engine

A Rust-based voxel engine using `wgpu` for rendering.

## Features

-   Voxel rendering with neighbor culling (6 faces).
-   Texture Array support.
-   Simple terrain generation (sine wave).
-   FPS Camera controls.
-   Asset downloading via Lua script.

## Requirements

-   Rust (stable 2021 or newer).
-   Vulkan, Metal, or DX12 compatible GPU.
-   Internet connection (first run) to download texture assets.

## Controls

-   **W, A, S, D**: Move
-   **Space**: Move Up
-   **Left Shift**: Move Down
-   **Mouse**: Look around
-   **Esc**: Close window (or Alt+F4)

## Building and Running

1.  Clone the repository.
2.  Run with cargo:

    ```bash
    cargo run --release
    ```

    *Note: The first run will download texture assets to `assets/textures/`.*

## Project Structure

-   `src/`: Rust source code.
-   `shaders/`: WGSL shaders.
-   `scripts/`: Lua scripts for assets and configuration.
