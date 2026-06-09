1. **Refactor `State::new` return type:**
   Change `State::new` in `src/state.rs` to return `anyhow::Result<Self>` instead of `Self`. This gracefully handles initialization errors without panicking.
2. **Implement parallel chunk terrain generation:**
   Generate a 3x3 grid of chunks. Use `rayon`'s `par_iter_mut()` to generate terrain across multiple chunks.
   Use a sine/cosine wave for height mapping: `height = (16.0 + (world_x * 0.1).sin() * 5.0 + (world_z * 0.1).cos() * 5.0) as usize`.
3. **Implement parallel mesh generation:**
   Use `rayon`'s `par_iter()` to generate meshes for the chunks.
   Combine the resulting meshes into a single vertex and index buffer. Use `let mut all_vertices: Vec<mesher::Vertex> = Vec::new();` and calculate index offsets using `vertex_count`.
4. **Update `main.rs`:**
   Handle the `Result` returned from `State::new` properly (e.g. `expect("Failed to init state")` or `unwrap()`).
5. **Ensure pre-commit and testing:**
   Complete pre-commit steps to make sure proper testing, verifications, reviews, and reflections are done.
6. **Submit changes:**
   Commit and submit the improvements.
