#include "rendering/VulkanRenderer.hpp"

#include <array>
#include <cstring>
#include <stdexcept>

#include "core/Window.hpp"

namespace lh::render {

void VulkanRenderer::update_uniform_buffer(std::uint32_t current_image, const glm::mat4& view,
                                           const glm::mat4& proj) {
    UniformBufferObject ubo{};
    ubo.view = view;
    ubo.proj = proj;
    ubo.light_dir = glm::vec4(glm::normalize(glm::vec3(-0.4f, -1.0f, -0.3f)), 0.0f);
    std::memcpy(uniform_mapped_[current_image], &ubo, sizeof(ubo));
}

void VulkanRenderer::record_command_buffer(VkCommandBuffer cmd, std::uint32_t image_index) {
    VkCommandBufferBeginInfo begin{};
    begin.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    if (vkBeginCommandBuffer(cmd, &begin) != VK_SUCCESS) {
        throw std::runtime_error("Failed to begin command buffer");
    }

    std::array<VkClearValue, 2> clear_values{};
    clear_values[0].color = {{0.53f, 0.68f, 0.85f, 1.0f}}; // sky blue
    clear_values[1].depthStencil = {1.0f, 0};

    VkRenderPassBeginInfo render_pass_info{};
    render_pass_info.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
    render_pass_info.renderPass = render_pass_;
    render_pass_info.framebuffer = swapchain_framebuffers_[image_index];
    render_pass_info.renderArea.offset = {0, 0};
    render_pass_info.renderArea.extent = swapchain_extent_;
    render_pass_info.clearValueCount = static_cast<std::uint32_t>(clear_values.size());
    render_pass_info.pClearValues = clear_values.data();

    vkCmdBeginRenderPass(cmd, &render_pass_info, VK_SUBPASS_CONTENTS_INLINE);
    vkCmdBindPipeline(cmd, VK_PIPELINE_BIND_POINT_GRAPHICS, graphics_pipeline_);

    VkViewport viewport{};
    viewport.width = static_cast<float>(swapchain_extent_.width);
    viewport.height = static_cast<float>(swapchain_extent_.height);
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;
    vkCmdSetViewport(cmd, 0, 1, &viewport);

    VkRect2D scissor{};
    scissor.extent = swapchain_extent_;
    vkCmdSetScissor(cmd, 0, 1, &scissor);

    vkCmdBindDescriptorSets(cmd, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline_layout_, 0, 1,
                            &descriptor_sets_[current_frame_], 0, nullptr);

    std::uint32_t visible = 0;
    for (const GpuMesh& mesh : meshes_) {
        if (mesh.index_count == 0) {
            continue;
        }
        // Skip meshes whose bounding box is entirely outside the view frustum.
        if (!frustum_.intersects_aabb(mesh.aabb_min, mesh.aabb_max)) {
            continue;
        }
        ++visible;
        const VkBuffer buffers[] = {mesh.vertex_buffer};
        const VkDeviceSize offsets[] = {0};
        vkCmdBindVertexBuffers(cmd, 0, 1, buffers, offsets);
        vkCmdBindIndexBuffer(cmd, mesh.index_buffer, 0, VK_INDEX_TYPE_UINT32);
        vkCmdDrawIndexed(cmd, mesh.index_count, 1, 0, 0, 0);
    }
    visible_count_ = visible;

    vkCmdEndRenderPass(cmd);
    if (vkEndCommandBuffer(cmd) != VK_SUCCESS) {
        throw std::runtime_error("Failed to record command buffer");
    }
}

void VulkanRenderer::draw_frame(const glm::mat4& view, const glm::mat4& proj) {
    vkWaitForFences(device_, 1, &in_flight_[current_frame_], VK_TRUE, UINT64_MAX);

    std::uint32_t image_index = 0;
    VkResult acquire = vkAcquireNextImageKHR(device_, swapchain_, UINT64_MAX,
                                             image_available_[current_frame_], VK_NULL_HANDLE,
                                             &image_index);
    if (acquire == VK_ERROR_OUT_OF_DATE_KHR) {
        recreate_swapchain();
        return;
    }
    if (acquire != VK_SUCCESS && acquire != VK_SUBOPTIMAL_KHR) {
        throw std::runtime_error("Failed to acquire swapchain image");
    }

    update_uniform_buffer(current_frame_, view, proj);
    frustum_.update(proj * view);

    vkResetFences(device_, 1, &in_flight_[current_frame_]);
    vkResetCommandBuffer(command_buffers_[current_frame_], 0);
    record_command_buffer(command_buffers_[current_frame_], image_index);

    VkSubmitInfo submit{};
    submit.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
    const VkSemaphore wait_semaphores[] = {image_available_[current_frame_]};
    const VkPipelineStageFlags wait_stages[] = {
        VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT};
    submit.waitSemaphoreCount = 1;
    submit.pWaitSemaphores = wait_semaphores;
    submit.pWaitDstStageMask = wait_stages;
    submit.commandBufferCount = 1;
    submit.pCommandBuffers = &command_buffers_[current_frame_];
    const VkSemaphore signal_semaphores[] = {render_finished_[current_frame_]};
    submit.signalSemaphoreCount = 1;
    submit.pSignalSemaphores = signal_semaphores;

    if (vkQueueSubmit(graphics_queue_, 1, &submit, in_flight_[current_frame_]) != VK_SUCCESS) {
        throw std::runtime_error("Failed to submit draw command buffer");
    }

    VkPresentInfoKHR present{};
    present.sType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
    present.waitSemaphoreCount = 1;
    present.pWaitSemaphores = signal_semaphores;
    const VkSwapchainKHR swapchains[] = {swapchain_};
    present.swapchainCount = 1;
    present.pSwapchains = swapchains;
    present.pImageIndices = &image_index;

    const VkResult present_result = vkQueuePresentKHR(present_queue_, &present);
    if (present_result == VK_ERROR_OUT_OF_DATE_KHR || present_result == VK_SUBOPTIMAL_KHR ||
        window_->was_resized()) {
        window_->reset_resized_flag();
        recreate_swapchain();
    } else if (present_result != VK_SUCCESS) {
        throw std::runtime_error("Failed to present swapchain image");
    }

    current_frame_ = (current_frame_ + 1) % kMaxFramesInFlight;
}

} // namespace lh::render
