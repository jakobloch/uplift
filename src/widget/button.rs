use ash::vk;
use crate::renderer::pipeline::Pipeline;
use crate::renderer::swapchain::Swapchain;

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub label: String,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self {
        Button {
            x,
            y,
            width,
            height,
            label: label.to_string(),
        }
    }

    pub fn draw(&self, device: &ash::Device, command_buffer: vk::CommandBuffer, pipeline: &Pipeline) {
        // For simplicity, we will draw the button as a colored rectangle.
        // In a real application, you would have a more complex method here.
        let vertices: [f32; 12] = [
            self.x, self.y, 0.0, 1.0, 0.0, 0.0, 1.0,
            self.x + self.width, self.y, 0.0, 0.0, 1.0, 0.0, 1.0,
            self.x + self.width, self.y + self.height, 0.0, 0.0, 0.0, 1.0, 1.0,
            self.x, self.y + self.height, 0.0, 1.0, 1.0, 0.0, 1.0,
        ];

        // Bind the pipeline
        unsafe {
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline.graphics_pipeline);
            // In a real application, you would bind the vertex buffer here
            device.cmd_draw(command_buffer, 6, 1, 0, 0);
        }
    }

    pub fn handle_click(&self, x: f32, y: f32) -> bool {
        if x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height {
            println!("Button '{}' clicked!", self.label);
            true
        } else {
            false
        }
    }
}
