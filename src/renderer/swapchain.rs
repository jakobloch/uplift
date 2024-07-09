use ash::khr::{self, surface, swapchain};
use ash::vk;
use ash::{Device, Instance};
use winit::window::Window;

pub struct SwapchainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

pub struct Swapchain {
    pub swapchain: khr::swapchain::Device,
    pub swapchain_khr: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

/// Represents a Vulkan swapchain.
impl Swapchain {
    /// Creates a new swapchain.
    ///
    /// # Arguments
    ///
    /// * `instance` - The Vulkan instance.
    /// * `physical_device` - The physical device.
    /// * `device` - The Vulkan device.
    /// * `surface` - The Vulkan surface.
    /// * `window` - The window.
    ///
    /// # Returns
    ///
    /// A new `Swapchain` instance.
    pub fn new(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        device: &Device,
        surface: vk::SurfaceKHR,
        window: &Window,
    ) -> Self {
        // Create a surface loader
        let surface_loader =
            surface::Instance::new(unsafe { &ash::Entry::load().unwrap() }, instance);

        // Get the surface capabilities
        let capabilities = unsafe {
            surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface)
                .unwrap()
        };

        // Get the surface formats
        let formats = unsafe {
            surface_loader
                .get_physical_device_surface_formats(physical_device, surface)
                .unwrap()
        };

        // Get the present modes
        let present_modes = unsafe {
            surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface)
                .unwrap()
        };

        // Choose the surface format
        let surface_format = formats
            .iter()
            .cloned()
            .find(|format| {
                format.format == vk::Format::B8G8R8A8_SRGB
                    && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or(formats[0]);

        // Choose the present mode
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        // Choose the extent
        let extent = match capabilities.current_extent.width {
            std::u32::MAX => {
                let window_size = window.inner_size();
                vk::Extent2D {
                    width: window_size.width,
                    height: window_size.height,
                }
            }
            _ => capabilities.current_extent,
        };

        // Choose the image count
        let image_count = if capabilities.max_image_count > 0 {
            capabilities
                .max_image_count
                .min(capabilities.min_image_count + 1)
        } else {
            capabilities.min_image_count + 1
        };

        // Create the swapchain create info
        let create_info = vk::SwapchainCreateInfoKHR {
            surface,
            min_image_count: image_count,
            image_format: surface_format.format,
            image_color_space: surface_format.color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            pre_transform: capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
            ..Default::default()
        };

        // Create the swapchain device
        let swapchain = swapchain::Device::new(instance, device);

        // Create the swapchain
        let swapchain_khr = unsafe { swapchain.create_swapchain(&create_info, None).unwrap() };

        // Get the swapchain images
        let images = unsafe { swapchain.get_swapchain_images(swapchain_khr).unwrap() };

        // Create the image views
        let image_views = images
            .iter()
            .map(|&image| {
                let create_view_info = vk::ImageViewCreateInfo {
                    image,
                    view_type: vk::ImageViewType::TYPE_2D,
                    format: surface_format.format,
                    components: vk::ComponentMapping {
                        r: vk::ComponentSwizzle::IDENTITY,
                        g: vk::ComponentSwizzle::IDENTITY,
                        b: vk::ComponentSwizzle::IDENTITY,
                        a: vk::ComponentSwizzle::IDENTITY,
                    },
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    ..Default::default()
                };

                unsafe {
                    device
                        .create_image_view(&create_view_info, None)
                        .expect("Failed to create image view")
                }
            })
            .collect();

        // Return the swapchain
        Swapchain {
            swapchain,
            swapchain_khr,
            images,
            image_views,
            format: surface_format.format,
            extent,
        }
    }

    /// Cleans up the swapchain.
    ///
    /// # Arguments
    ///
    /// * `device` - The Vulkan device.
    pub fn cleanup(&self, device: &Device) {
        // Destroy the image views
        for &image_view in self.image_views.iter() {
            unsafe {
                device.destroy_image_view(image_view, None);
            }
        }

        // Destroy the swapchain
        unsafe {
            self.swapchain.destroy_swapchain(self.swapchain_khr, None);
        }
    }
}
