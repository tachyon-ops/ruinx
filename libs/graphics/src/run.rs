use std::time::Instant;

use futures::executor::block_on;

use winit::{
    dpi::PhysicalSize,
    event::{self, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    //
    engine::RenderError,
    // AppMode,
    Engine,
    GuiTrait,
    State,
};

// Generate the winit <-> conrod_core type conversion fns.
conrod_winit::v023_conversion_fns!();

struct App {
    engine: Box<dyn Engine>,
    state: Option<State>,
}

impl App {
    fn new(mut engine: Box<dyn Engine>) -> App {
        engine.setup();
        App {
            engine,
            state: None,
        }
    }

    // fn get_mode(&mut self) -> AppMode {
    //     self.engine.get_mode()
    // }

    fn update(&mut self) {
        match &mut self.state {
            Some(s) => s.update(),
            _ => {}
        }
        self.engine.update();
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        eprintln!("Resizing");
        match &mut self.state {
            Some(s) => s.resize(new_size),
            _ => {}
        }
        self.engine.resize(new_size);
    }

    fn set_state(&mut self, state: Option<State>) {
        match state {
            Some(s) => self.state = Some(s),
            None => self.state = None,
        }
    }

    fn has_state(&mut self) -> bool {
        self.state.is_some()
    }

    fn event_handler(&mut self, evt: &Event<()>, conrod_evt: conrod_core::event::Input) -> bool {
        self.engine.event(evt);
        if let Some(s) = &mut self.state {
            s.ui_handle_event(conrod_evt)
        }
        false
    }

    fn render(&mut self) -> Result<(), RenderError> {
        self.engine.update();
        match &mut self.state {
            Some(s) => s.render(),
            _ => Err(RenderError::MissplacedCall),
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

    let mut app = App::new(engine);
    app.set_state(state);

    let sixteen_ms = std::time::Duration::from_millis(16);
    let mut next_update: Option<Instant> = None;
    let mut ui_update_needed = false;

    log::info!("    --- EVENT LOOP ---");
    event_loop.run(move |event, _, control_flow| {
        // if app.get_mode() == AppMode::APP {
        //     // *control_flow = ControlFlow::Wait;

        //     if let Some(next_update) = next_update {
        //         *control_flow = ControlFlow::WaitUntil(next_update);
        //     } else {
        //         *control_flow = ControlFlow::Wait;
        //     }
        // }
        // if app.get_mode() == AppMode::GAME {
        //     if let Some(next_update) = next_update {
        //         *control_flow = ControlFlow::WaitUntil(next_update);
        //     } else {
        //         *control_flow = ControlFlow::Poll;
        //     }
        // }
        if app.has_state() {
            if let Some(conrod_event) = convert_event(&event, &window) {
                app.event_handler(&event, conrod_event);
                ui_update_needed = true;
            }

            match &event {
                event::Event::Suspended => {
                    log::info!("App suspended");
                    app.set_state(None);
                }

                event::Event::WindowEvent { event, .. } => match event {
                    // Recreate swapchain when window is resized.
                    WindowEvent::Resized(physical_size) => app.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        scale_factor,
                    } => {
                        log::info!("Scale Factor Changed: {}", scale_factor);
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
                },
                _ => {}
            }

            // We don't want to draw any faster than 60 FPS, so set the UI only on every 16ms, unless:
            // - this is the very first event, or
            // - we didn't request update on the last event and new events have arrived since then.
            let should_set_ui_on_main_events_cleared = next_update.is_none() && ui_update_needed;
            match (&event, should_set_ui_on_main_events_cleared) {
                (event::Event::NewEvents(event::StartCause::Init { .. }), _)
                | (event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }), _)
                | (event::Event::MainEventsCleared, true) => {
                    next_update = Some(std::time::Instant::now() + sixteen_ms);
                    ui_update_needed = false;

                    // Instantiate a GUI demonstrating every widget type provided by conrod.
                    // conrod_example_shared::gui(&mut ui.set_widgets(), &ids, &mut app);

                    app.update();

                    if app.ui_has_changed() {
                        // If the view has changed at all, request a redraw.
                        window.request_redraw();
                    } else {
                        // We don't need to update the UI anymore until more events arrives.
                        next_update = None;
                    }
                }
                _ => (),
            }
            if let Some(next_update) = next_update {
                *control_flow = ControlFlow::WaitUntil(next_update);
            } else {
                *control_flow = ControlFlow::Wait;
            }

            match &event {
                event::Event::RedrawRequested(_) => {
                    match app.render() {
                        _ => {}
                    };
                }
                _ => {}
            }

            // app.event(&event);

            // if let Some(event) = convert_event(&event, &window) {
            //     app.ui_handle_event(event);
            //     ui_update_needed = true;
            // }

            // match event {
            //     Event::WindowEvent {
            //         ref event,
            //         window_id,
            //     } if window_id == window.id() => {
            //         if !app.input(event) {
            //             match event {
            //                 WindowEvent::Resized(physical_size) => app.resize(*physical_size),
            //                 WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            //                     app.resize(**new_inner_size)
            //                 }
            //                 // Close on request or on Escape.
            //                 event::WindowEvent::KeyboardInput {
            //                     input:
            //                         event::KeyboardInput {
            //                             virtual_keycode: Some(event::VirtualKeyCode::Escape),
            //                             state: event::ElementState::Pressed,
            //                             ..
            //                         },
            //                     ..
            //                 }
            //                 | event::WindowEvent::CloseRequested => {
            //                     *control_flow = ControlFlow::Exit;
            //                     return;
            //                 }
            //                 _ => {}
            //             }
            //         }
            //     }
            //     Event::Suspended => {
            //         log::info!("App suspended");
            //         app.init(None);
            //     }
            //     _ => {}
            // };

            // let should_set_ui_on_main_events_cleared = next_update.is_none() && ui_update_needed;
            // match (&event, should_set_ui_on_main_events_cleared) {
            //     (event::Event::NewEvents(event::StartCause::Init { .. }), _)
            //     | (event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }), _)
            //     | (event::Event::MainEventsCleared, true) => {
            //         next_update = Some(std::time::Instant::now() + sixteen_ms);
            //         ui_update_needed = false;

            //         // // Instantiate a GUI demonstrating every widget type provided by conrod.
            //         // conrod_example_shared::gui(&mut ui.set_widgets(), &ids, &mut app);

            //         // if ui.has_changed() {
            //         //     // If the view has changed at all, request a redraw.
            //         //     window.request_redraw();
            //         // } else {
            //         //     // We don't need to update the UI anymore until more events arrives.
            //         //     next_update = None;
            //         // }

            //         // Instantiate a GUI demonstrating every widget type provided by conrod.
            //         // conrod_example_shared::gui(&mut ui.set_widgets(), &ids, &mut app);
            //         app.update();
            //         app.gui();

            //         if app.ui_has_changed() {
            //             // If the view has changed at all, request a redraw.
            //             window.request_redraw();
            //         } else {
            //             // We don't need to update the UI anymore until more events arrives.
            //             next_update = None;
            //         }
            //     }
            //     _ => (),
            // }

            // if let Some(next_update) = next_update {
            //     *control_flow = ControlFlow::WaitUntil(next_update);
            // } else {
            //     *control_flow = ControlFlow::Wait;
            // }

            // match &event {
            //     event::Event::RedrawRequested(_) => {
            //         // If the view has changed at all, request a redraw.
            //         match app.render(window.scale_factor()) {
            //             Ok(_) => {}
            //             Err(RenderError::SurfaceError(wgpu::SurfaceError::Lost)) => {
            //                 app.resize(app.size)
            //             }
            //             Err(RenderError::SurfaceError(wgpu::SurfaceError::OutOfMemory)) => {
            //                 *control_flow = ControlFlow::Exit
            //             }
            //             Err(e) => eprintln!("{:?}", e),
            //         }
            //     }
            //     _ => {}
            // }
        } else {
            match event {
                Event::Resumed => {
                    log::info!("App resumed");
                    app.set_state(Some(block_on(State::new(&window, gui.clone()))));
                    app.update();
                }
                _ => {}
            }
        }
    });
}
