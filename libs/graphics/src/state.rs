use conrod_core::Ui;
use conrod_wgpu::Image;
use wgpu::TextureView;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{GuiTrait, RenderError};

const MSAA_SAMPLES: u32 = 4;

fn create_multisampled_framebuffer(
    device: &wgpu::Device,
    sc_desc: &wgpu::SwapChainDescriptor,
    sample_count: u32,
) -> wgpu::TextureView {
    let multisampled_texture_extent = wgpu::Extent3d {
        width: sc_desc.width,
        height: sc_desc.height,
        depth_or_array_layers: 1,
    };
    let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
        label: Some("conrod_msaa_texture"),
        size: multisampled_texture_extent,
        mip_level_count: 1,
        sample_count: sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: sc_desc.format,
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
    };
    device
        .create_texture(multisampled_frame_descriptor)
        .create_view(&wgpu::TextureViewDescriptor::default())
}

pub struct State {
    pub size: PhysicalSize<u32>,
    surface: wgpu::Surface,
    // adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    image_map: conrod_core::image::Map<Image>,
    multisampled_framebuffer: TextureView,
    renderer: conrod_wgpu::Renderer,
    gui: Box<dyn GuiTrait>,
    ui: Ui,
}

impl State {
    pub async fn new(
        window: &Window,
        gui: Box<dyn GuiTrait>,
        win_size: winit::dpi::LogicalSize<f64>,
    ) -> Self {
        eprintln!("State::new");
        log::info!("----------------------------------------- Activating!");

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
        let (device, mut queue) = adapter
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

        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let renderer = conrod_wgpu::Renderer::new(&device, MSAA_SAMPLES, swapchain_format);

        let multisampled_framebuffer =
            create_multisampled_framebuffer(&device, &sc_desc, MSAA_SAMPLES);

        let image_map = conrod_core::image::Map::new();

        eprint!("Generating UI\n");
        let mut gui = gui;
        let ui = conrod_core::UiBuilder::new([win_size.width, win_size.height])
            .theme(gui.theme())
            .build();

        let ui = gui.init(ui, &device, &mut queue, swapchain_format);

        Self {
            size,
            surface,
            // adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            multisampled_framebuffer,
            renderer,
            image_map,
            gui,
            ui,
        }
    }

    pub fn gui(&mut self) {
        // Instantiate a GUI demonstrating every widget type provided by conrod.
        // conrod_example_shared::gui(&mut ui.set_widgets(), &ids, &mut app);
        self.gui.gui(&mut self.ui.set_widgets());
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        log::info!("Resizing: {} x {}", new_size.width, new_size.height);

        // Recreate the swap chain with the new size
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.multisampled_framebuffer =
            create_multisampled_framebuffer(&self.device, &self.sc_desc, MSAA_SAMPLES);

        // self.render();
    }

    pub fn update(&mut self) {
        //     if let Some(ui) = &mut self.ui {
        //         ui.has_changed();
        //     }
    }

    pub fn ui_handle_event(&mut self, event: conrod_core::event::Input) {
        println!("State::ui_handle_event: event = {:?}", event);
        self.ui.handle_event(event);
    }

    pub fn ui_has_changed(&mut self) -> bool {
        return self.ui.has_changed();
    }

    pub fn render(&mut self, scale_factor: f64) -> Result<(), RenderError> {
        let primitives = self.ui.draw();

        // The window frame that we will draw to.
        let frame = self.swap_chain.get_current_frame().unwrap();

        // Begin encoding commands.
        let cmd_encoder_desc = wgpu::CommandEncoderDescriptor {
            label: Some("conrod_command_encoder"),
        };
        let mut encoder = self.device.create_command_encoder(&cmd_encoder_desc);

        // Feed the renderer primitives and update glyph cache texture if necessary.
        // let scale_factor = window.scale_factor();
        let [win_w, win_h]: [f32; 2] = [self.size.width as f32, self.size.height as f32];
        let viewport = [0.0, 0.0, win_w, win_h];
        if let Some(cmd) = self
            .renderer
            .fill(&self.image_map, viewport, scale_factor, primitives)
            .unwrap()
        {
            cmd.load_buffer_and_encode(&self.device, &mut encoder);
        }

        // Begin the render pass and add the draw commands.
        {
            // This condition allows to more easily tweak the MSAA_SAMPLES constant.
            let (attachment, resolve_target) = match MSAA_SAMPLES {
                1 => (&frame.output.view, None),
                _ => (&self.multisampled_framebuffer, Some(&frame.output.view)),
            };
            let color_attachment_desc = wgpu::RenderPassColorAttachment {
                view: attachment,
                resolve_target,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            };

            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("conrod_render_pass_descriptor"),
                color_attachments: &[color_attachment_desc],
                depth_stencil_attachment: None,
            };
            let render = self.renderer.render(&self.device, &self.image_map);

            {
                let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
                let slot = 0;
                render_pass.set_vertex_buffer(slot, render.vertex_buffer.slice(..));
                let instance_range = 0..1;
                for cmd in render.commands {
                    match cmd {
                        conrod_wgpu::RenderPassCommand::SetPipeline { pipeline } => {
                            render_pass.set_pipeline(pipeline);
                        }
                        conrod_wgpu::RenderPassCommand::SetBindGroup { bind_group } => {
                            render_pass.set_bind_group(0, bind_group, &[]);
                        }
                        conrod_wgpu::RenderPassCommand::SetScissor {
                            top_left,
                            dimensions,
                        } => {
                            let [x, y] = top_left;
                            let [w, h] = dimensions;
                            render_pass.set_scissor_rect(x, y, w, h);
                        }
                        conrod_wgpu::RenderPassCommand::Draw { vertex_range } => {
                            render_pass.draw(vertex_range, instance_range.clone());
                        }
                    }
                }
            }
        }

        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}
