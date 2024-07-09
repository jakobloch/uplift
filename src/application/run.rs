use ash::vk;
use ash::Entry;
use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::renderer::command::{create_sync_objects, CommandBuffers, CommandPool};
use crate::renderer::device::AshDevice;
use crate::renderer::framebuffer::Framebuffers;
use crate::renderer::instance::AshInstance;
use crate::renderer::pipeline::Pipeline;
use crate::renderer::render_pass::RenderPass;
use crate::renderer::swapchain::Swapchain;

pub fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Failed to create window.");

    let entry = Entry::linked();
    let instance = AshInstance::new(&entry, "Ash Application", &window);
    let device = AshDevice::new(&Arc::new(instance.instance));

    let swapchain = Swapchain::new(
        &instance.instance,
        &device.device,
        device.physical_device,
        &window,
    );

    let render_pass = RenderPass::new(
        &device.device,
        swapchain.swapchain_format,
        vk::Format::D32_SFLOAT, // Depth format, should be queried from the GPU
    );

    let framebuffers = Framebuffers::new(
        &device.device,
        render_pass.render_pass,
        &swapchain.image_views,
        swapchain.extent,
    );

    let command_pool = CommandPool::new(
        &device.device,
        device.queue_family_indices.graphics_family.unwrap(),
    );
    let command_buffers = CommandBuffers::new(
        &device.device,
        &command_pool,
        framebuffers.framebuffers.len() as u32,
    );

    let graphics_pipeline = Pipeline::new(
        &device.device,
        render_pass.render_pass,
        swapchain.extent,
    );

    let max_frames_in_flight = 2;
    let (image_available_semaphores, render_finished_semaphores, in_flight_fences) =
        create_sync_objects(&device.device, max_frames_in_flight);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                // Redraw the application
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Rendering code
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });

    // Cleanup
    graphics_pipeline.cleanup(&device.device);
    command_pool.cleanup(&device.device);
    framebuffers.cleanup(&device.device);
    render_pass.cleanup(&device.device);
    swapchain.cleanup(&device.device);
    device.cleanup();
    instance.cleanup();
}

fn load_shader_module(device: &ash::Device, path: &str) -> vk::ShaderModule {
    let code = std::fs::read(path).expect("Failed to read shader code.");
    let shader_info = vk::ShaderModuleCreateInfo::default().code(&code);
    unsafe {
        device
            .create_shader_module(&shader_info, None)
            .expect("Failed to create shader module.")
    }
}
