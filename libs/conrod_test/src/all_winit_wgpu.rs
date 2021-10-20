//! An example demonstrating the use of `conrod_wgpu` alongside `winit`.

use conrod_example_shared::{WIN_H, WIN_W};
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
};

use crate::state::State;

// Generate the winit <-> conrod_core type conversion fns.
conrod_winit::v023_conversion_fns!();

pub fn run_conrod() {
    let event_loop = EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("Conrod with wgpu")
        .with_inner_size(winit::dpi::LogicalSize {
            width: WIN_W,
            height: WIN_H,
        })
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_os = "android"))]
    let mut state = Some(State::new(&window));
    #[cfg(target_os = "android")]
    let mut state: Option<crate::state::State> = None;

    let sixteen_ms = std::time::Duration::from_millis(16);
    let mut next_update = None;
    let mut ui_update_needed = false;
    event_loop.run(move |event, _, control_flow| {
        match &mut state {
            Some(s) => {
                if let Some(event) = convert_event(&event, &window) {
                    // ui.handle_event(event);
                    s.update(event);
                    ui_update_needed = true;
                }

                match &event {
                    event::Event::WindowEvent { event, .. } => match event {
                        // Recreate swapchain when window is resized.
                        event::WindowEvent::Resized(new_size) => {
                            s.resize(new_size);
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
                let should_set_ui_on_main_events_cleared =
                    next_update.is_none() && ui_update_needed;
                match (&event, should_set_ui_on_main_events_cleared) {
                    (event::Event::NewEvents(event::StartCause::Init { .. }), _)
                    | (event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }), _)
                    | (event::Event::MainEventsCleared, true) => {
                        next_update = Some(std::time::Instant::now() + sixteen_ms);
                        ui_update_needed = false;

                        s.init_gui();

                        if s.has_changed() {
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
                        s.render(&window);
                    }
                    _ => {}
                }
            }
            _ => {
                match event {
                    event::Event::Resumed => {
                        // log::info!("App resumed");
                        state = Some(State::new(&window));
                    }
                    _ => {}
                }
            }
        };
    });
}
