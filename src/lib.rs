use futures::executor::block_on;

use std::borrow::Cow;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct State {
    size: PhysicalSize<u32>,
    surface: wgpu::Surface,
    // adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    async fn new(window: &Window) -> Self {
        log::info!("----------------------------------------- Activating!");
        log::info!("Activating now!");

        let size = window.inner_size();
        log::info!("Size: {} x {}", size.width, size.height);

        log::info!("Instance");
        let instance = wgpu::Instance::new(wgpu::BackendBit::all());

        log::info!("Surface");
        let surface = unsafe { instance.create_surface(window) };
        log::info!("Adapter");
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        log::info!("Device and Queue!");
        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    // limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        log::info!("Shader");

        // Load the shaders from disk
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        log::info!("Pipeline layout");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        log::info!("Swapchain format");

        // OpenGL BUG (simulator): rust_jsx_ui: Swapchain format
        // I/RustStdoutStderr: thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value', /Users/nmpribeiro/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.9.2/src/hub.rs:878:29
        //     stack backtrace:
        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

        log::info!("Render pipeline");
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[swapchain_format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        log::info!("Swapchain description");
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        log::info!("Swapchain");
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            size,
            surface,
            // adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            render_pipeline,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        log::info!("Resizing: {} x {}", size.width, size.height);
        // Recreate the swap chain with the new size
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        // self.render();
    }

    fn update(&mut self) {}

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self
            .swap_chain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}

fn sentry_init() {
    let _guard = sentry::init((
        "https://examplePublicKey@o0.ingest.sentry.io/0",
        sentry::ClientOptions {
            release: Some("my-project-name@2.3.12".into()),
            // OR automatically:
            // release: sentry::release_name!(),
            ..Default::default()
        },
    ));
}

pub fn entry() {
    sentry_init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(not(target_os = "android"))]
    let mut state_ = Some(block_on(State::new(&window)));
    #[cfg(target_os = "android")]
    let mut state_: std::option::Option<State> = None;

    log::info!("    --- EVENT LOOP ---");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
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
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
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

/// Bindings entry point
#[no_mangle]
pub extern "C" fn main_rs() {
    env_logger::init();
    entry();
}

// ANativeActivity_onCreate not found: undefined symbol: ANativeActivity_onCreate
// Backtrace can be "on" or "full"
#[cfg(target_os = "android")]
#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "rust-app"))
)]
fn android_entry() {
    entry();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    entry();
}
