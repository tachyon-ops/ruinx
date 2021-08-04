use std::borrow::Cow;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

struct Extent2D {
    width: u32,
    height: u32,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const DIMS: Extent2D = Extent2D { width: 1024, height: 768 };

fn wait_for_native_window() {
    log::info!("Will now wait for native window");
    #[cfg(target_os = "android")]
    {
        log::info!("App started. Waiting for NativeScreen");
        loop {
            match ndk_glue::native_window().as_ref() {
                Some(_) => {
                    log::info!("NativeScreen Found:{:?}", ndk_glue::native_window());
                    break;
                }
                None => (),
            }
        }
    }
    log::info!("Proceeding after native window");
}

// fn block_until_has_size(window: &Window) -> PhysicalSize<u32> {
//     let mut size = window.inner_size();

//     let mut condition = true;
//     let mut i = 0;
//     while condition {
//         std::thread::sleep(std::time::Duration::from_millis(250));
//         // wait_for_native_window();
//         size = window.inner_size();
//         log::info!("Window size is {} x {}", size.width, size.height);

//         if size.width != 0 {
//             condition = false;
//         }
//         if i >= 20 {
//             condition = false;
//         }
//         i = i + 1;
//     }
//     log::info!("Window size is {} x {}", size.width, size.height);
//     size
// }

struct State {
    window: Window,

    suspended: bool,
    surface: Option<wgpu::Surface>,
    adapter: Option<wgpu::Adapter>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    sc_desc: Option<wgpu::SwapChainDescriptor>,
    swap_chain: Option<wgpu::SwapChain>,
    render_pipeline: Option<wgpu::RenderPipeline>,
}

impl State {
    fn new(window: Window) -> Self {
        Self {
            window,
            suspended: true,
            surface: None,
            adapter: None,
            device: None,
            queue: None,
            sc_desc: None,
            swap_chain: None,
            render_pipeline: None,
        }
    }

    async fn get_surface_adapter_device(&mut self) {
        if self.suspended {
            return;
        }
        wait_for_native_window();
        log::info!("Instance");

        let instance;
        #[cfg(target_os = "android")]
        {
            log::info!("------------- Selecting VULKAN -------------");
            instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        }

        #[cfg(not(target_os = "android"))]
        {
            instance = wgpu::Instance::new(wgpu::Backends::all());
        }

        log::info!("Surface");
        let surface = unsafe { instance.create_surface(&self.window) };
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
                    limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        self.surface = Some(surface);
        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);
    }

    async fn activate(&mut self) {
        log::info!("----------------------------------------- Activating!");
        log::info!("Activating now!");

        let size = self.window.inner_size();
        log::info!("Size: {} x {}", size.width, size.height);

        wait_for_native_window();

        self.get_surface_adapter_device().await;

        log::info!("Shader");

        let device;
        let adapter;
        match &self.device {
            Some(d) => device = d,
            None => {
                println!("No device found! Will try again next loop iteration.");
                self.suspend();
                return;
            }
        }
        match &self.adapter {
            Some(a) => adapter = a,
            None => {
                println!("No adapter found! Will try again next loop iteration.");
                self.suspend();
                return;
            }
        }

        // Load the shaders from disk
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        log::info!("Pipeline layout");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Check if surface exists
        let surface;
        match &self.surface {
            Some(s) => surface = s,
            None => {
                println!("No surface found! Will try again next loop iteration.");
                self.suspend();
                return;
            }
        }

        log::info!("Swapchain format");
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        log::info!("Swapchain");
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        self.sc_desc = Some(sc_desc);
        self.swap_chain = Some(swap_chain);
        self.render_pipeline = Some(render_pipeline);

        self.suspended = false;
    }

    fn suspend(&mut self) {
        log::info!("----------------------------------------- Suspending!");
        self.surface = None;
        self.adapter = None;
        self.device = None;
        self.queue = None;
        self.sc_desc = None;
        self.swap_chain = None;
        self.render_pipeline = None;
        self.suspended = true;
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        log::info!("Resizeing: {} x {}", size.width, size.height);
        // Recreate the swap chain with the new size
        let surface;
        let device;
        let mut sc_desc;

        match &self.surface {
            Some(val) => surface = val,
            None => {
                println!("No surface found during resize!");
                self.suspend();
                return;
            }
        }
        match &self.device {
            Some(val) => device = val,
            None => {
                println!("No device found during resize!");
                self.suspend();
                return;
            }
        }

        match &self.sc_desc {
            Some(val) => sc_desc = val.to_owned(),
            None => {
                println!("No sc_desc found during resize!");
                self.suspend();
                return;
            }
        }
        sc_desc.width = size.width;
        sc_desc.height = size.height;
        self.swap_chain = Some(device.create_swap_chain(&surface, &sc_desc));
        self.render();
    }

    fn render(&mut self) {
        let swap_chain;
        let device;
        let render_pipeline;
        let queue;

        match &self.swap_chain {
            Some(val) => swap_chain = val,
            None => {
                self.suspend();
                return;
            }
        }

        match &self.device {
            Some(val) => device = val,
            None => {
                self.suspend();
                return;
            }
        }

        match &self.render_pipeline {
            Some(val) => render_pipeline = val,
            None => {
                self.suspend();
                return;
            }
        }

        match &self.queue {
            Some(val) => queue = val,
            None => {
                self.suspend();
                return;
            }
        }

        if self.suspended {
            return;
        }

        let frame = swap_chain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
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
            rpass.set_pipeline(&render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
    }
}

pub fn run() {
    initialize_logger();

    let event_loop = winit::event_loop::EventLoop::new();
    let wb = winit::window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(
            64.0, 64.0,
        )))
        .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize::new(
            DIMS.width,
            DIMS.height,
        )))
        .with_title("quad".to_string());

    // instantiate backend
    let window = wb.build(&event_loop).unwrap();

    wait_for_native_window();

    let mut state = State::new(window);

    log::info!("    --- EVENT LOOP ---");
    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        // let _ = (&instance, &adapter, &shader, &pipeline_layout);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::Resumed => {
                state.activate();
            }
            Event::Suspended => {
                state.suspend();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                state.resize(size);
            }
            Event::RedrawRequested(_) => {
                state.render();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        } // end of match event

        if state.suspended {
            state.activate();
        }
    });
}

fn initialize_logger() {
    #[cfg(target_os = "android")]
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }

    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();
}

/// Bindings entry point
#[no_mangle]
pub extern "C" fn main_rs() {
    run();
}

// ANativeActivity_onCreate not found: undefined symbol: ANativeActivity_onCreate
// Backtrace can be "on" or "full"
#[cfg(target_os = "android")]
#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "rust-app"))
)]
fn android_entry() {
    run();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    run();
}
