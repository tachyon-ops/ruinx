use iced_native::{Color, Program};

pub trait Application: Program<Renderer = iced_graphics::Renderer<iced_wgpu::Backend>> {
    fn background_color(&self) -> Color;

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        false
    }
}
