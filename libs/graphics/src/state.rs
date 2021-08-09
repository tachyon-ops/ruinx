use std::borrow::Cow;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

pub struct State {
    pub size: PhysicalSize<u32>,
    surface: wgpu::Surface,
    // adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    pub async fn new(window: &Window) -> Self {
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

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        log::info!("Resizing: {} x {}", size.width, size.height);
        // Recreate the swap chain with the new size
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        // self.render();
    }

    pub fn update(&mut self) {}

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
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
