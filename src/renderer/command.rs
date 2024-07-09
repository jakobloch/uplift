use ash::vk;
use ash::Device;
use std::sync::Arc;

pub struct CommandPool {
    pub pool: vk::CommandPool,
}

impl CommandPool {
    pub fn new(device: &Arc<Device>, queue_family_index: u32) -> Self {
        let pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let pool = unsafe {
            device
                .create_command_pool(&pool_info, None)
                .expect("Failed to create command pool!")
        };

        CommandPool { pool }
    }

    pub fn cleanup(&self, device: &Arc<Device>) {
        unsafe {
            device.destroy_command_pool(self.pool, None);
        }
    }
}

pub struct CommandBuffers {
    pub buffers: Vec<vk::CommandBuffer>,
}

impl CommandBuffers {
    pub fn new(device: &Arc<Device>, command_pool: &CommandPool, count: u32) -> Self {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool.pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count);

        let buffers = unsafe {
            device
                .allocate_command_buffers(&alloc_info)
                .expect("Failed to allocate command buffers!")
        };

        CommandBuffers { buffers }
    }

    pub fn begin_command_buffer(&self, buffer_index: usize) {
        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

        unsafe {
            self.device()
                .begin_command_buffer(self.buffers[buffer_index], &begin_info)
                .expect("Failed to begin recording command buffer!");
        }
    }

    pub fn end_command_buffer(&self, buffer_index: usize) {
        unsafe {
            self.device()
                .end_command_buffer(self.buffers[buffer_index])
                .expect("Failed to end recording command buffer!");
        }
    }

    pub fn device(&self) -> &Arc<Device> {
        // Assuming CommandBuffers is part of a larger structure that includes a reference to the device.
        // You may need to modify this function to get the actual device reference.
        unimplemented!()
    }

    pub fn record_commands(
        &self,
        buffer_index: usize,
        render_pass: vk::RenderPass,
        framebuffer: vk::Framebuffer,
        extent: vk::Extent2D,
    ) {
        let command_buffer = self.buffers[buffer_index];

        let begin_info = vk::CommandBufferBeginInfo::default();

        unsafe {
            self.device()
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("Failed to begin recording command buffer!");
        }

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];

        let render_pass_info = vk::RenderPassBeginInfo::default()
            .render_pass(render_pass)
            .framebuffer(framebuffer)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent,
            })
            .clear_values(&clear_values);

        unsafe {
            self.device().cmd_begin_render_pass(
                command_buffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );

            // Record drawing commands here

            self.device().cmd_end_render_pass(command_buffer);

            self.device()
                .end_command_buffer(command_buffer)
                .expect("Failed to record command buffer!");
        }
    }
}

pub fn create_sync_objects(
    device: &Arc<Device>,
    max_frames_in_flight: usize,
) -> (Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>) {
    let semaphore_info = vk::SemaphoreCreateInfo::default();

    let mut image_available_semaphores = Vec::with_capacity(max_frames_in_flight);
    let mut render_finished_semaphores = Vec::with_capacity(max_frames_in_flight);
    let mut in_flight_fences = Vec::with_capacity(max_frames_in_flight);

    let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

    for _ in 0..max_frames_in_flight {
        unsafe {
            image_available_semaphores.push(
                device
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore!"),
            );
            render_finished_semaphores.push(
                device
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore!"),
            );
            in_flight_fences.push(
                device
                    .create_fence(&fence_info, None)
                    .expect("Failed to create fence!"),
            );
        }
    }

    (
        image_available_semaphores,
        render_finished_semaphores,
        in_flight_fences,
    )
}
