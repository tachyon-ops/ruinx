use iced_native::{Command, Element, Program};

#[derive(Debug, Clone)]
pub enum RuinXMessage {
    BackgroundColorChanged(iced_winit::Color),
}
pub trait IcedProgramTrait {
    fn update(&mut self, message: RuinXMessage) -> Command<RuinXMessage>;
    fn view(&mut self) -> Element<'_, RuinXMessage, iced_wgpu::Renderer>;
}

impl Program for dyn IcedProgramTrait
where
    Self: Sized,
{
    type Renderer = iced_wgpu::Renderer;

    type Message = RuinXMessage;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        todo!()
    }

    fn view(&mut self) -> Element<'_, Self::Message, Self::Renderer> {
        todo!()
    }
}
