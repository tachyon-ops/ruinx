use crate::AppMode;

pub trait Stage {
    fn get_mode(&mut self) -> AppMode;
    fn setup(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    // fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {}
}
