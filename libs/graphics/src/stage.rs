use std::fmt::Display;

use conrod_core::render::Primitives;
use winit::{dpi::PhysicalSize, event::Event};

use crate::AppMode;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RenderError {
    SwapChainError(wgpu::SwapChainError),
    MissplacedCall,
    Unknown,
}

impl From<wgpu::SwapChainError> for RenderError {
    fn from(error: wgpu::SwapChainError) -> RenderError {
        RenderError::SwapChainError(error)
    }
}

impl Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SwapChainError(inner) => write!(f, "{}", inner),
            Self::MissplacedCall => write!(f, "Render function was called in the wrong place"),
            _ => write!(f, "Unknown render error"),
        }
    }
}

pub trait Stage {
    fn get_mode(&mut self) -> AppMode;
    fn setup(&mut self);
    fn update(&mut self);
    fn render(&mut self) -> Result<Primitives, RenderError>;
    fn event(&mut self, event: &Event<()>);
    fn resize(&mut self, size: PhysicalSize<u32>);
}
