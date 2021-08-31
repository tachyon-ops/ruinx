use futures::executor::block_on;

use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{stage::RenderError, AppMode, Stage, State};

struct App {
    size: PhysicalSize<u32>,
    stage: Box<dyn Stage>,
    state: Option<State>,
}

impl App {
    fn new(stage: Box<dyn Stage>, state: Option<State>, size: PhysicalSize<u32>) -> App {
        App { stage, state, size }
    }
    fn get_mode(&mut self) -> AppMode {
        self.stage.get_mode()
    }
    fn setup(&mut self) {
        self.stage.setup();
    }
    fn update(&mut self) {
        match &mut self.state {
            Some(s) => s.update(),
            _ => {}
        }
        self.stage.update();
    }
    fn event(&mut self, event: &Event<()>) {
        self.stage.event(event);
    }
    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        match &mut self.state {
            Some(s) => s.resize(new_size),
            _ => {}
        }
        self.stage.resize(new_size);
    }
    fn set_state(&mut self, state: Option<State>) {
        self.state = state;
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
                self.stage.render()?;
                s.render(scale_factor, self.stage.render()?)
            }
            _ => Err(RenderError::MissplacedCall),
        }
    }
}

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn event_loop(stage: Box<dyn Stage>) {
    log::info!("Inside RUN!");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(APP_NAME)
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_os = "android"))]
    let state_ = Some(block_on(State::new(&window)));
    #[cfg(target_os = "android")]
    let state_: std::option::Option<State> = None;

    let mut app = App::new(stage, state_, window.inner_size());

    app.setup();

    log::info!("    --- EVENT LOOP ---");

    event_loop.run(move |event, _, control_flow| {
        if app.get_mode() == AppMode::APP {
            *control_flow = ControlFlow::Wait;
        }
        if app.get_mode() == AppMode::GAME {
            *control_flow = ControlFlow::Poll;
        }
        if app.has_state() {
            app.event(&event);
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !app.input(event) {
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
                            WindowEvent::Resized(physical_size) => app.resize(*physical_size),
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                app.resize(**new_inner_size)
                            }
                            _ => {}
                        }
                    }
                }
                Event::MainEventsCleared => {
                    app.update();
                    match app.render(window.scale_factor()) {
                        Ok(_) => {}
                        Err(RenderError::SwapChainError(wgpu::SwapChainError::Lost)) => {
                            app.resize(app.size)
                        }
                        Err(RenderError::SwapChainError(wgpu::SwapChainError::OutOfMemory)) => {
                            *control_flow = ControlFlow::Exit
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::Suspended => {
                    log::info!("App suspended");
                    app.set_state(None);
                }
                _ => {}
            };
        } else {
            match event {
                Event::Resumed => {
                    log::info!("App resumed");
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    app.set_state(Some(block_on(State::new(&window))));
                }
                _ => {}
            }
        }
    });
}
