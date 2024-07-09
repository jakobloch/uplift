use ash::vk;
use std::sync::Arc;
use ash::khr::surface::Instance as SurfaceInstance;

use super::instance::AshInstance;

pub struct AshDevice<'a> {
    pub instance: &'a AshInstance,
    pub physical_device: vk::PhysicalDevice,
    pub device: Arc<ash::Device>,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub queue_family_indices: QueueFamilyIndices,
}

impl<'a> AshDevice<'a> {
    pub fn new(instance: &'a AshInstance) -> Self {
        let physical_device = AshDevice::pick_physical_device(instance);
        let queue_family_indices = AshDevice::find_queue_families(instance, physical_device);
        let (device, graphics_queue, present_queue) = AshDevice::create_logical_device(
            instance,
            physical_device,
            &queue_family_indices,
        );

        AshDevice {
            instance,
            physical_device,
            device: Arc::new(device),
            graphics_queue,
            present_queue,
            queue_family_indices,
        }
    }

    fn pick_physical_device(instance: &AshInstance) -> vk::PhysicalDevice {
        let physical_devices = unsafe {
            instance.instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices!")
        };

        physical_devices
            .into_iter()
            .find(|&device| AshDevice::is_device_suitable(instance, device))
            .expect("Failed to find a suitable GPU!")
    }

    fn is_device_suitable(instance: &AshInstance, device: vk::PhysicalDevice) -> bool {
        let indices = AshDevice::find_queue_families(instance, device);
        indices.is_complete()
    }

    fn find_queue_families(instance: &AshInstance, device: vk::PhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::default();

        let queue_families = unsafe {
            &instance.instance
                .get_physical_device_queue_family_properties(device)
        };

        for (i, queue_family) in queue_families.iter().enumerate() {
            if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                indices.graphics_family = Some(i as u32);
            }

            let present_support = unsafe {
                SurfaceInstance::new(&instance.entry, &instance.instance)
                    .get_physical_device_surface_support(device, i as u32, vk::SurfaceKHR::null())
                    .unwrap_or(false)
            };

            if present_support {
                indices.present_family = Some(i as u32);
            }

            if indices.is_complete() {
                break;
            }
        }

        indices
    }

    fn create_logical_device(
        instance: &AshInstance,
        physical_device: vk::PhysicalDevice,
        indices: &QueueFamilyIndices,
    ) -> (ash::Device, vk::Queue, vk::Queue) {
        let queue_priorities = [1.0f32];

        let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = vec![
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(indices.graphics_family.unwrap())
                .queue_priorities(&queue_priorities),
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(indices.present_family.unwrap())
                .queue_priorities(&queue_priorities),
        ];

        let device_extensions = [ash::khr::swapchain::NAME.as_ptr()];

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(&device_extensions);

        let device = unsafe {
            instance.instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device!")
        };

        let graphics_queue = unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };
        let present_queue = unsafe { device.get_device_queue(indices.present_family.unwrap(), 0) };

        (device, graphics_queue, present_queue)
    }
}

#[derive(Default)]
pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

impl QueueFamilyIndices {
    fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}
