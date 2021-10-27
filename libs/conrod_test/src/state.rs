use conrod_core::{image::Map, Ui};
use conrod_example_shared::{WIN_H, WIN_W};
use conrod_wgpu::{Image, Renderer};
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureView};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{create_multisampled_framebuffer, MSAA_SAMPLES};

pub struct State {
    ui: Ui,
    surface: Surface,
    device: Device,
    size: PhysicalSize<u32>,
    renderer: Renderer,
    image_map: Map<Image>,
    multisampled_framebuffer: TextureView,
    queue: Queue,
    surface_config: SurfaceConfiguration,
    // ids: conrod_example_shared::Ids,
    ids: crate::conrod_example::Ids,
    // app: conrod_example_shared::DemoApp,
    app: crate::conrod_example::DemoApp,
}

impl State {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter =
            futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            }))
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, mut queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("conrod_device_descriptor"),
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits:
                    // wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
                    wgpu::Limits::default().using_resolution(adapter.limits()),
            },
            None,
        ))
        .expect("Failed to create device");

        // Create the swapchain.
        let format = surface.get_preferred_format(&adapter).unwrap();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        // Create the renderer for rendering conrod primitives.
        let renderer = conrod_wgpu::Renderer::new(&device, MSAA_SAMPLES, format);

        // The intermediary multisampled texture that will be resolved (MSAA).
        let multisampled_framebuffer =
            create_multisampled_framebuffer(&device, &surface_config, MSAA_SAMPLES);

        // Create Ui and Ids of widgets to instantiate
        let mut ui = conrod_core::UiBuilder::new([WIN_W as f64, WIN_H as f64])
            .theme(conrod_example_shared::theme())
            .build();
        // let ids = conrod_example_shared::Ids::new(ui.widget_id_generator());
        let ids = crate::conrod_example::Ids::new(ui.widget_id_generator());

        // Load font from file
        let font_path = "fonts/NotoSans/NotoSans-Regular.ttf";
        let font = crate::assets::load_font(font_path);
        ui.fonts.insert(font);

        // // Load the Rust logo from our assets folder to use as an example image.F
        let logo = "images/rust.png";
        let rgba_logo_image = crate::assets::load_image(logo).to_rgba8();

        // Create the GPU texture and upload the image data.
        let (logo_w, logo_h) = rgba_logo_image.dimensions();
        let logo_tex = crate::create_logo_texture(&device, &mut queue, rgba_logo_image);
        let logo = conrod_wgpu::Image {
            texture: logo_tex,
            texture_format: crate::LOGO_TEXTURE_FORMAT,
            width: logo_w,
            height: logo_h,
        };
        let mut image_map = conrod_core::image::Map::new();
        let rust_logo = image_map.insert(logo);

        // Demonstration app state that we'll control with our conrod GUI.
        let app = crate::conrod_example::DemoApp::new(rust_logo);
        // let app = crate::conrod_example::DemoApp::new();

        Self {
            ui,
            surface,
            device,
            size,
            renderer,
            image_map,
            multisampled_framebuffer,
            queue,
            surface_config,
            ids,
            app,
        }
    }

    pub fn update(&mut self, event: conrod_core::event::Input) {
        self.ui.handle_event(event);
    }

    pub fn render(&mut self, window: &Window) {
        let primitives = self.ui.draw();

        // The window frame that we will draw to.
        let surface_texture = self.surface.get_current_texture().unwrap();

        // Begin encoding commands.
        let cmd_encoder_desc = wgpu::CommandEncoderDescriptor {
            label: Some("conrod_command_encoder"),
        };
        let mut encoder = self.device.create_command_encoder(&cmd_encoder_desc);

        // Feed the renderer primitives and update glyph cache texture if necessary.
        let scale_factor = window.scale_factor();
        let [win_w, win_h]: [f32; 2] = [self.size.width as f32, self.size.height as f32];
        let viewport = [0.0, 0.0, win_w, win_h];
        if let Some(cmd) = self
            .renderer
            .fill(&self.image_map, viewport, scale_factor, primitives)
            .unwrap()
        {
            cmd.load_buffer_and_encode(&self.device, &mut encoder);
        }

        // Create a view for the surface's texture.

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Begin the render pass and add the draw commands.
        {
            // This condition allows to more easily tweak the MSAA_SAMPLES constant.
            let (attachment, resolve_target) = match MSAA_SAMPLES {
                1 => (&surface_view, None),
                _ => (&self.multisampled_framebuffer, Some(&surface_view)),
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
        surface_texture.present();
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.size = *new_size;
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.multisampled_framebuffer =
            create_multisampled_framebuffer(&self.device, &self.surface_config, MSAA_SAMPLES);
    }

    pub fn init_gui(&mut self) {
        // Instantiate a GUI demonstrating every widget type provided by conrod.
        // conrod_example_shared::gui(&mut self.ui.set_widgets(), &self.ids, &mut self.app);
        crate::conrod_example::gui(&mut self.ui.set_widgets(), &self.ids, &mut self.app);
    }

    pub fn has_changed(&mut self) -> bool {
        self.ui.has_changed()
    }
}
