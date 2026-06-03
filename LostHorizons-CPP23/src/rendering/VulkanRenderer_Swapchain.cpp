#include "rendering/VulkanRenderer.hpp"

#include <algorithm>
#include <array>
#include <cstdint>
#include <limits>
#include <stdexcept>

#include "core/Window.hpp"

namespace lh::render {

namespace {

VkSurfaceFormatKHR choose_surface_format(const std::vector<VkSurfaceFormatKHR>& formats) {
    for (const auto& format : formats) {
        if (format.format == VK_FORMAT_B8G8R8A8_SRGB &&
            format.colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR) {
            return format;
        }
    }
    return formats.front();
}

VkPresentModeKHR choose_present_mode(const std::vector<VkPresentModeKHR>& modes) {
    for (const auto& mode : modes) {
        if (mode == VK_PRESENT_MODE_MAILBOX_KHR) {
            return mode;
        }
    }
    return VK_PRESENT_MODE_FIFO_KHR; // always available
}

} // namespace

VulkanRenderer::SwapchainSupport
VulkanRenderer::query_swapchain_support(VkPhysicalDevice device) const {
    SwapchainSupport support;
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR(device, surface_, &support.capabilities);

    std::uint32_t format_count = 0;
    vkGetPhysicalDeviceSurfaceFormatsKHR(device, surface_, &format_count, nullptr);
    support.formats.resize(format_count);
    vkGetPhysicalDeviceSurfaceFormatsKHR(device, surface_, &format_count, support.formats.data());

    std::uint32_t mode_count = 0;
    vkGetPhysicalDeviceSurfacePresentModesKHR(device, surface_, &mode_count, nullptr);
    support.present_modes.resize(mode_count);
    vkGetPhysicalDeviceSurfacePresentModesKHR(device, surface_, &mode_count,
                                              support.present_modes.data());
    return support;
}

void VulkanRenderer::create_swapchain() {
    const SwapchainSupport support = query_swapchain_support(physical_device_);
    const VkSurfaceFormatKHR surface_format = choose_surface_format(support.formats);
    const VkPresentModeKHR present_mode = choose_present_mode(support.present_modes);

    VkExtent2D extent = support.capabilities.currentExtent;
    if (extent.width == std::numeric_limits<std::uint32_t>::max()) {
        int width = 0;
        int height = 0;
        window_->framebuffer_size(width, height);
        extent.width = std::clamp(static_cast<std::uint32_t>(width),
                                  support.capabilities.minImageExtent.width,
                                  support.capabilities.maxImageExtent.width);
        extent.height = std::clamp(static_cast<std::uint32_t>(height),
                                   support.capabilities.minImageExtent.height,
                                   support.capabilities.maxImageExtent.height);
    }

    std::uint32_t image_count = support.capabilities.minImageCount + 1;
    if (support.capabilities.maxImageCount > 0 &&
        image_count > support.capabilities.maxImageCount) {
        image_count = support.capabilities.maxImageCount;
    }

    VkSwapchainCreateInfoKHR info{};
    info.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
    info.surface = surface_;
    info.minImageCount = image_count;
    info.imageFormat = surface_format.format;
    info.imageColorSpace = surface_format.colorSpace;
    info.imageExtent = extent;
    info.imageArrayLayers = 1;
    info.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;

    const std::array<std::uint32_t, 2> families = {graphics_family_, present_family_};
    if (graphics_family_ != present_family_) {
        info.imageSharingMode = VK_SHARING_MODE_CONCURRENT;
        info.queueFamilyIndexCount = 2;
        info.pQueueFamilyIndices = families.data();
    } else {
        info.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
    }

    info.preTransform = support.capabilities.currentTransform;
    info.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
    info.presentMode = present_mode;
    info.clipped = VK_TRUE;
    info.oldSwapchain = VK_NULL_HANDLE;

    if (vkCreateSwapchainKHR(device_, &info, nullptr, &swapchain_) != VK_SUCCESS) {
        throw std::runtime_error("Failed to create swapchain");
    }

    vkGetSwapchainImagesKHR(device_, swapchain_, &image_count, nullptr);
    swapchain_images_.resize(image_count);
    vkGetSwapchainImagesKHR(device_, swapchain_, &image_count, swapchain_images_.data());
    swapchain_format_ = surface_format.format;
    swapchain_extent_ = extent;
}

void VulkanRenderer::create_image_views() {
    swapchain_image_views_.resize(swapchain_images_.size());
    for (std::size_t i = 0; i < swapchain_images_.size(); ++i) {
        VkImageViewCreateInfo info{};
        info.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        info.image = swapchain_images_[i];
        info.viewType = VK_IMAGE_VIEW_TYPE_2D;
        info.format = swapchain_format_;
        info.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        info.subresourceRange.levelCount = 1;
        info.subresourceRange.layerCount = 1;
        if (vkCreateImageView(device_, &info, nullptr, &swapchain_image_views_[i]) !=
            VK_SUCCESS) {
            throw std::runtime_error("Failed to create image view");
        }
    }
}

VkFormat VulkanRenderer::find_depth_format() const {
    const std::array<VkFormat, 3> candidates = {
        VK_FORMAT_D32_SFLOAT, VK_FORMAT_D32_SFLOAT_S8_UINT, VK_FORMAT_D24_UNORM_S8_UINT};
    for (VkFormat format : candidates) {
        VkFormatProperties props{};
        vkGetPhysicalDeviceFormatProperties(physical_device_, format, &props);
        if (props.optimalTilingFeatures & VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT) {
            return format;
        }
    }
    throw std::runtime_error("Failed to find a supported depth format");
}

void VulkanRenderer::create_depth_resources() {
    depth_format_ = find_depth_format();

    VkImageCreateInfo image_info{};
    image_info.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
    image_info.imageType = VK_IMAGE_TYPE_2D;
    image_info.extent = {swapchain_extent_.width, swapchain_extent_.height, 1};
    image_info.mipLevels = 1;
    image_info.arrayLayers = 1;
    image_info.format = depth_format_;
    image_info.tiling = VK_IMAGE_TILING_OPTIMAL;
    image_info.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    image_info.usage = VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;
    image_info.samples = VK_SAMPLE_COUNT_1_BIT;
    image_info.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    if (vkCreateImage(device_, &image_info, nullptr, &depth_image_) != VK_SUCCESS) {
        throw std::runtime_error("Failed to create depth image");
    }

    VkMemoryRequirements requirements{};
    vkGetImageMemoryRequirements(device_, depth_image_, &requirements);
    VkMemoryAllocateInfo alloc{};
    alloc.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    alloc.allocationSize = requirements.size;
    alloc.memoryTypeIndex =
        find_memory_type(requirements.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    if (vkAllocateMemory(device_, &alloc, nullptr, &depth_memory_) != VK_SUCCESS) {
        throw std::runtime_error("Failed to allocate depth memory");
    }
    vkBindImageMemory(device_, depth_image_, depth_memory_, 0);

    VkImageViewCreateInfo view_info{};
    view_info.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    view_info.image = depth_image_;
    view_info.viewType = VK_IMAGE_VIEW_TYPE_2D;
    view_info.format = depth_format_;
    view_info.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
    view_info.subresourceRange.levelCount = 1;
    view_info.subresourceRange.layerCount = 1;
    if (vkCreateImageView(device_, &view_info, nullptr, &depth_view_) != VK_SUCCESS) {
        throw std::runtime_error("Failed to create depth image view");
    }
}

void VulkanRenderer::create_framebuffers() {
    swapchain_framebuffers_.resize(swapchain_image_views_.size());
    for (std::size_t i = 0; i < swapchain_image_views_.size(); ++i) {
        const std::array<VkImageView, 2> attachments = {swapchain_image_views_[i], depth_view_};
        VkFramebufferCreateInfo info{};
        info.sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        info.renderPass = render_pass_;
        info.attachmentCount = static_cast<std::uint32_t>(attachments.size());
        info.pAttachments = attachments.data();
        info.width = swapchain_extent_.width;
        info.height = swapchain_extent_.height;
        info.layers = 1;
        if (vkCreateFramebuffer(device_, &info, nullptr, &swapchain_framebuffers_[i]) !=
            VK_SUCCESS) {
            throw std::runtime_error("Failed to create framebuffer");
        }
    }
}

void VulkanRenderer::cleanup_swapchain() {
    vkDestroyImageView(device_, depth_view_, nullptr);
    vkDestroyImage(device_, depth_image_, nullptr);
    vkFreeMemory(device_, depth_memory_, nullptr);

    for (VkFramebuffer fb : swapchain_framebuffers_) {
        vkDestroyFramebuffer(device_, fb, nullptr);
    }
    for (VkImageView view : swapchain_image_views_) {
        vkDestroyImageView(device_, view, nullptr);
    }
    vkDestroySwapchainKHR(device_, swapchain_, nullptr);
}

void VulkanRenderer::recreate_swapchain() {
    int width = 0;
    int height = 0;
    window_->framebuffer_size(width, height);
    while (width == 0 || height == 0) {
        window_->framebuffer_size(width, height);
        glfwWaitEvents();
    }

    vkDeviceWaitIdle(device_);
    cleanup_swapchain();

    create_swapchain();
    create_image_views();
    create_depth_resources();
    create_framebuffers();
}

} // namespace lh::render
