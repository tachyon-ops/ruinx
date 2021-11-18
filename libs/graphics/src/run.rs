use futures::executor::block_on;

use iced_wgpu::Viewport;
use iced_winit::{
    futures,
    winit::{self, event::ModifiersState, window::Window},
    Size,
};

use winit::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{engine::RenderError, iced_program_trait::IcedProgramTrait, State};

struct App {
    // engine: Box<dyn Engine>,
    state: Option<State>,
}

impl App {
    // fn new(mut engine: Box<dyn Engine>) -> App {
    fn new() -> App {
        // engine.setup();
        App { state: None }
    }

    fn resize(&mut self, viewport: Viewport) {
        eprintln!("Resizing");
        match &mut self.state {
            Some(s) => s.resize(viewport),
            _ => {}
        }
        // self.engine.resize(new_size);
    }

    fn set_state(&mut self, state: Option<State>) {
        match state {
            Some(s) => self.state = Some(s),
            None => self.state = None,
        }
    }

    // fn has_state(&mut self) -> bool {
    //     self.state.is_some()
    // }

    fn render(&mut self, window: &Window) -> Result<(), RenderError> {
        // self.engine.update();
        match &mut self.state {
            Some(s) => s.render(window),
            _ => Err(RenderError::MissplacedCall),
        }
    }

    fn set_cursor_position(&mut self, position: PhysicalPosition<f64>) {
        match &mut self.state {
            Some(s) => s.set_cursor_position(position),
            _ => {}
        }
    }

    fn queue_event(&mut self, event: iced_winit::Event) {
        match &mut self.state {
            Some(s) => s.queue_event(event),
            _ => {}
        }
    }

    fn is_queue_empty(&mut self) -> bool {
        match &mut self.state {
            Some(s) => s.is_queue_empty(),
            _ => true,
        }
    }

    fn update(&mut self) {
        match &mut self.state {
            Some(s) => s.update(),
            _ => {}
        }
    }
}

// pub fn event_loop(name: &'static str, engine: Box<dyn Engine>, gui: Box<dyn GuiTrait>) {
pub fn event_loop(name: &'static str) {
    log::info!("Inside RUN!");
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(name)
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_os = "android"))]
    let state = Some(block_on(State::new(&window)));
    // let state = Some(block_on(State::new(&window, gui.clone())));

    #[cfg(target_os = "android")]
    let state: std::option::Option<State> = None;

    // let mut app = App::new(engine);
    let mut app = App::new();
    app.set_state(state);

    let mut modifiers = ModifiersState::default();

    log::info!("    --- EVENT LOOP ---");
    event_loop.run(move |event, _, control_flow| {
        // You should change this if you want to render continuosly
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        app.set_cursor_position(position);
                    }
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        modifiers = new_modifiers;
                    }
                    WindowEvent::Resized(new_size) => {
                        app.resize(Viewport::with_physical_size(
                            Size::new(new_size.width, new_size.height),
                            window.scale_factor(),
                        ));
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }

                // Map window event to iced event
                if let Some(event) =
                    iced_winit::conversion::window_event(&event, window.scale_factor(), modifiers)
                {
                    // state.queue_event(event);
                    app.queue_event(event);
                }
            }
            Event::MainEventsCleared => {
                // If there are events pending
                if !app.is_queue_empty() {
                    // We update iced
                    let _ = app.update();
                    // and request a redraw
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                let _ = app.render(&window);
            }
            _ => {}
        };
    });
}
