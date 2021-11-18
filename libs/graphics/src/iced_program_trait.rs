use iced_native::{Command, Element};

#[derive(Debug, Clone)]
pub enum RuinXMessage {
    BackgroundColorChanged(iced_winit::Color),
}
pub trait IcedProgramTrait {
    fn update(&mut self, message: RuinXMessage) -> Command<RuinXMessage>;
    fn view(&mut self) -> Element<'_, RuinXMessage, iced_wgpu::Renderer>;
}
