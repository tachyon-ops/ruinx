use std::time::Instant;

use futures::executor::block_on;

use winit::{
    dpi::PhysicalSize,
    event::{self, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{engine::RenderError, AppMode, Engine, GuiTrait, State};

// Generate the winit <-> conrod_core type conversion fns.
conrod_winit::v023_conversion_fns!();

struct App {
    size: PhysicalSize<u32>,
    engine: Box<dyn Engine>,
    state: Option<State>,
}

impl App {
    fn new(mut engine: Box<dyn Engine>, size: PhysicalSize<u32>) -> App {
        engine.setup();
        App {
            size,
            engine,
            state: None,
        }
    }

    fn get_mode(&mut self) -> AppMode {
        self.engine.get_mode()
    }

    fn update(&mut self) {
        match &mut self.state {
            Some(s) => s.update(),
            _ => {}
        }
        self.engine.update();
    }

    fn event(&mut self, event: &Event<()>) {
        self.engine.event(event);
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        match &mut self.state {
            Some(s) => s.resize(new_size),
            _ => {}
        }
        self.engine.resize(new_size);
    }

    fn init(&mut self, state: Option<State>) {
        eprintln!("Set state");
        if let Some(mut s) = state {
            s.generate_ui();
            self.state = Some(s);
        }
    }

    fn has_state(&mut self) -> bool {
        self.state.is_some()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match &mut self.state {
            Some(s) => s.input(event),
            _ => false,
        }
    }

    fn render(&mut self, scale_factor: f64) -> Result<(), RenderError> {
        match &mut self.state {
            Some(s) => {
                self.engine.update();
                s.render(scale_factor)
            }
            _ => Err(RenderError::MissplacedCall),
        }
    }

    fn ui_handle_event(&mut self, event: conrod_core::event::Input) {
        if let Some(s) = &mut self.state {
            s.ui_handle_event(event);
        }
    }

    fn ui_has_changed(&mut self) -> bool {
        if let Some(s) = &mut self.state {
            return s.ui_has_changed();
        }
        false
    }
}

pub fn event_loop(name: &'static str, engine: Box<dyn Engine>, gui: Box<dyn GuiTrait>) {
    log::info!("Inside RUN!");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(name)
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_os = "android"))]
    let state = Some(block_on(State::new(&window, gui.clone())));
    #[cfg(target_os = "android")]
    let state: std::option::Option<State> = None;

    let mut app = App::new(engine, window.inner_size());
    app.init(state);

    log::info!("    --- EVENT LOOP ---");

    let sixteen_ms = std::time::Duration::from_millis(16);
    let mut next_update: Option<Instant> = None;
    let mut ui_update_needed = false;

    event_loop.run(move |event, _, control_flow| {
        if app.get_mode() == AppMode::APP {
            // *control_flow = ControlFlow::Wait;

            if let Some(next_update) = next_update {
                *control_flow = ControlFlow::WaitUntil(next_update);
            } else {
                *control_flow = ControlFlow::Wait;
            }
        }
        if app.get_mode() == AppMode::GAME {
            if let Some(next_update) = next_update {
                *control_flow = ControlFlow::WaitUntil(next_update);
            } else {
                *control_flow = ControlFlow::Poll;
            }
        }
        if app.has_state() {
            app.event(&event);

            if let Some(event) = convert_event(&event, &window) {
                app.ui_handle_event(event);
                ui_update_needed = true;
            }

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !app.input(event) {
                        match event {
                            WindowEvent::Resized(physical_size) => app.resize(*physical_size),
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                app.resize(**new_inner_size)
                            }
                            // Close on request or on Escape.
                            event::WindowEvent::KeyboardInput {
                                input:
                                    event::KeyboardInput {
                                        virtual_keycode: Some(event::VirtualKeyCode::Escape),
                                        state: event::ElementState::Pressed,
                                        ..
                                    },
                                ..
                            }
                            | event::WindowEvent::CloseRequested => {
                                *control_flow = ControlFlow::Exit;
                                return;
                            }
                            _ => {}
                        }
                    }
                }

                Event::MainEventsCleared => {
                    let should_set_ui_on_main_events_cleared =
                        next_update.is_none() && ui_update_needed;

                    if should_set_ui_on_main_events_cleared {
                        next_update = Some(std::time::Instant::now() + sixteen_ms);
                        ui_update_needed = false;

                        // Instantiate a GUI demonstrating every widget type provided by conrod.
                        // conrod_example_shared::gui(&mut ui.set_widgets(), &ids, &mut app);
                        app.update();

                        if app.ui_has_changed() {
                            // If the view has changed at all, request a redraw.
                            match app.render(window.scale_factor()) {
                                Ok(_) => {}
                                Err(RenderError::SwapChainError(wgpu::SwapChainError::Lost)) => {
                                    app.resize(app.size)
                                }
                                Err(RenderError::SwapChainError(
                                    wgpu::SwapChainError::OutOfMemory,
                                )) => *control_flow = ControlFlow::Exit,
                                Err(e) => eprintln!("{:?}", e),
                            }
                        } else {
                            // We don't need to update the UI anymore until more events arrives.
                            next_update = None;
                        }
                    }
                }
                Event::Suspended => {
                    log::info!("App suspended");
                    app.init(None);
                }
                _ => {}
            };
        } else {
            match event {
                Event::Resumed => {
                    log::info!("App resumed");
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    app.init(Some(block_on(State::new(&window, gui.clone()))));
                }
                _ => {}
            }
        }
    });
}
