pub mod view;
pub mod view_properties;

// use sdl2::{render::Canvas, video::Window};
use uuid::Uuid;
use view::View;

#[derive(Default)]
pub struct UiDom {
    pub root: View,
}

impl UiDom {
    pub fn new(size: (u32, u32)) -> Self {
        let mut root = view::View::new();
        root.set_id("root");
        root.set_origin(0, 0);
        root.set_dimensions(size.0, size.1);
        Self { root }
    }

    pub fn add(&mut self, view: View) {
        self.root.add(view);
    }

    pub fn get_view_in_point(&mut self, x: u32, y: u32) -> Uuid {
        self.root.get_views_in_point(x, y)
    }

    // pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
    pub fn draw(&mut self) {
        // draw root
        // &self.root.draw(canvas);
        todo!();
    }
}
