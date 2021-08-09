use futures::executor::block_on;

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{AppMode, Stage, State};

struct App {
    stage: Box<dyn Stage>,
}

impl App {
    fn new(stage: Box<dyn Stage>) -> App {
        App { stage }
    }
    fn get_mode(&mut self) -> AppMode {
        self.stage.get_mode()
    }
    fn setup(&mut self) {
        self.stage.setup();
    }
    fn update(&mut self) {
        self.stage.update();
    }
    fn render(&mut self) {
        self.stage.render();
    }
}

pub fn event_loop(stage: Box<dyn Stage>) {
    log::info!("Inside RUN!");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(not(target_os = "android"))]
    let mut state_ = Some(block_on(State::new(&window)));
    #[cfg(target_os = "android")]
    let mut state_: std::option::Option<State> = None;

    let mut app = App::new(stage);

    app.setup();

    log::info!("    --- EVENT LOOP ---");
    event_loop.run(move |event, _, control_flow| {
        if app.get_mode() == AppMode::APP {
            *control_flow = ControlFlow::Wait;
        }
        match &mut state_ {
            Some(state) => match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            WindowEvent::KeyboardInput { input, .. } => match input {
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {}
                            },
                            WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::MainEventsCleared => {
                    state.update();
                    app.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                    app.render();
                }
                Event::Suspended => {
                    log::info!("App suspended");
                    state_ = None;
                }
                _ => {}
            },
            None => match event {
                Event::Resumed => {
                    log::info!("App resumed");
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    state_ = Some(block_on(State::new(&window)));
                }
                _ => {}
            },
        }
    });
}
