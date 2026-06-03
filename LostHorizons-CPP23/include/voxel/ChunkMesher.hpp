#pragma once

#include <functional>
#include "rendering/Vertex.hpp"
#include "voxel/VoxelData.hpp"

namespace lh::voxel {

// Converts a chunk's voxels into a renderable triangle mesh using simple
// face culling: a cube face is only emitted when the adjacent voxel is not
// solid. Vertices are baked in world space so the renderer can use an
// identity model matrix.
class ChunkMesher {
public:
    // Returns true for solid voxels at chunk-local coordinates. Coordinates may
    // be out of [0, kChunkSize) so the caller can resolve neighbouring chunks
    // and keep chunk borders seamless.
    using SolidSampler = std::function<bool(int x, int y, int z)>;

    [[nodiscard]] static render::MeshData build(const Chunk& chunk,
                                                const SolidSampler& solid);
};

} // namespace lh::voxel
