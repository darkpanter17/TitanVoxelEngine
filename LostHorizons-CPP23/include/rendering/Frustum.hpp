#pragma once

#include <array>

#include <glm/glm.hpp>

namespace lh::render {

// View frustum extracted from a view-projection matrix (Gribb-Hartmann).
// Used to cull chunk meshes whose bounding box lies entirely outside the view.
class Frustum {
public:
    // `vp` is the combined projection * view matrix. The project uses a
    // zero-to-one depth range (GLM_FORCE_DEPTH_ZERO_TO_ONE), so the near plane
    // is row 2 (rather than row3 + row2 as in OpenGL's [-1, 1] range).
    void update(const glm::mat4& vp) {
        const auto row = [&](int i) {
            return glm::vec4(vp[0][i], vp[1][i], vp[2][i], vp[3][i]);
        };
        planes_[0] = row(3) + row(0); // left
        planes_[1] = row(3) - row(0); // right
        planes_[2] = row(3) + row(1); // bottom
        planes_[3] = row(3) - row(1); // top
        planes_[4] = row(2);          // near
        planes_[5] = row(3) - row(2); // far
    }

    // True if the axis-aligned box [mn, mx] is at least partially inside the
    // frustum. Uses the "positive vertex" test: a box is culled only when it is
    // fully on the negative side of any single plane.
    [[nodiscard]] bool intersects_aabb(const glm::vec3& mn, const glm::vec3& mx) const {
        for (const glm::vec4& p : planes_) {
            const glm::vec3 positive(p.x >= 0.0f ? mx.x : mn.x,
                                     p.y >= 0.0f ? mx.y : mn.y,
                                     p.z >= 0.0f ? mx.z : mn.z);
            if (p.x * positive.x + p.y * positive.y + p.z * positive.z + p.w < 0.0f) {
                return false;
            }
        }
        return true;
    }

private:
    std::array<glm::vec4, 6> planes_{};
};

} // namespace lh::render
