// use std::fmt::Display;

use iced_winit::winit::{dpi::PhysicalSize, event::Event};

use crate::AppMode;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RenderError {
    SurfaceError, // (wgpu::SurfaceError),
    MissplacedCall,
    Unknown,
}

// impl From<wgpu::SurfaceError> for RenderError {
//     fn from(error: wgpu::SurfaceError) -> RenderError {
//         RenderError::SurfaceError(error)
//     }
// }

// impl Display for RenderError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::SurfaceError(inner) => write!(f, "{}", inner),
//             Self::MissplacedCall => write!(f, "Render function was called in the wrong place"),
//             _ => write!(f, "Unknown render error"),
//         }
//     }
// }

pub trait Engine {
    fn get_mode(&mut self) -> AppMode;
    fn setup(&mut self);
    fn update(&mut self);
    fn render(&mut self) -> Result<(), RenderError>;
    fn event(&mut self, event: &Event<()>);
    fn resize(&mut self, size: PhysicalSize<u32>);
}
