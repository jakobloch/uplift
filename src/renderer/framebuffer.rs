use ash::vk;
use ash::Device;
use std::sync::Arc;

pub struct Framebuffers {
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl Framebuffers {
    pub fn new(
        device: &Arc<Device>,
        render_pass: vk::RenderPass,
        image_views: &[vk::ImageView],
        depth_image_view: vk::ImageView,
        swapchain_extent: vk::Extent2D,
    ) -> Framebuffers {
        let mut framebuffers = Vec::with_capacity(image_views.len());

        for &image_view in image_views {
            let attachments = [image_view, depth_image_view];
            let framebuffer_info = vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(swapchain_extent.width)
                .height(swapchain_extent.height)
                .layers(1);

            let framebuffer = unsafe {
                device
                    .create_framebuffer(&framebuffer_info, None)
                    .expect("Failed to create framebuffer!")
            };

            framebuffers.push(framebuffer);
        }

        Framebuffers { framebuffers }
    }

    pub fn cleanup(&self, device: &Arc<Device>) {
        for &framebuffer in &self.framebuffers {
            unsafe {
                device.destroy_framebuffer(framebuffer, None);
            }
        }
    }
}
