use iced_winit::winit;

#[derive(Debug, Clone)]
pub enum AppEvent {
    // TODO: add app events if needed (so far, winit works fine?)
}

#[derive(Debug)]
pub enum Event<'a> {
    App(AppEvent),
    Wintit(&'a winit::event::Event<'a, ()>),
}
