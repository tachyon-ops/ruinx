// use conrod_core::Ui;
// use conrod_wgpu::Image;
// use wgpu::TextureView;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    window::Window,
};

// use crate::{GuiTrait, RenderError};

use iced_wgpu::{wgpu, Backend, Renderer, Settings, Viewport};
use iced_winit::{conversion, futures, program, winit, Clipboard, Debug, Size};

// const MSAA_SAMPLES: u32 = 4;

// pub fn get_win_size(window: &Window) -> (LogicalSize<f64>, f64) {
//     // #[cfg(not(target_os = "android"))]
//     let scale_factor = window.scale_factor();
//     // #[cfg(target_os = "android")]
//     // let scale_factor = 3.0;

//     log::info!("Scale factor: {}", scale_factor);
//     let size = window.inner_size();
//     log::info!("Size: {} x {}", size.width, size.height);
//     (size.to_logical(scale_factor), scale_factor)
// }

// fn create_multisampled_framebuffer(
//     device: &wgpu::Device,
//     surface_config: &wgpu::SurfaceConfiguration,
//     sample_count: u32,
// ) -> wgpu::TextureView {
//     let multisampled_texture_extent = wgpu::Extent3d {
//         width: surface_config.width,
//         height: surface_config.height,
//         depth_or_array_layers: 1,
//     };
//     let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
//         label: Some("conrod_msaa_texture"),
//         size: multisampled_texture_extent,
//         mip_level_count: 1,
//         sample_count: sample_count,
//         dimension: wgpu::TextureDimension::D2,
//         format: surface_config.format,
//         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//     };
//     device
//         .create_texture(multisampled_frame_descriptor)
//         .create_view(&wgpu::TextureViewDescriptor::default())
// }

pub struct State {
    // pub size: PhysicalSize<u32>,
// scale_factor: f64,
// surface: wgpu::Surface,
// // adapter: wgpu::Adapter,
// device: wgpu::Device,
// queue: wgpu::Queue,
// surface_config: wgpu::SurfaceConfiguration,
// image_map: conrod_core::image::Map<Image>,
// multisampled_framebuffer: TextureView,
// renderer: conrod_wgpu::Renderer,
// gui: Box<dyn GuiTrait>,
// ui: Ui,
}

fn wait_for_native_window() {
    log::info!("Will now wait for native window");
    #[cfg(target_arch = "android")]
    {
        log::info!("Waiting for NativeScreen");
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
    log::info!("Proceeding after native window found");
}

impl State {
    pub async fn new(window: &Window, gui: Box<dyn GuiTrait>) -> Self {
        wait_for_native_window();

        let physical_size = window.inner_size();
        let mut viewport = Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            window.scale_factor(),
        );
        let mut cursor_position = PhysicalPosition::new(-1.0, -1.0);
        let mut modifiers = ModifiersState::default();
        let mut clipboard = Clipboard::connect(&window);

        // Initialize wgpu
        log::info!("Instance");
        #[cfg(not(target_os = "android"))]
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        #[cfg(target_os = "android")]
        let instance = {
            wgpu::Instance::new(wgpu::Backends::all())
            // wgpu::Instance::new(wgpu::Backends::GL)
        };

        log::info!("Surface");
        let surface = unsafe { instance.create_surface(window) };

        // // Select an adapter and gpu device.
        // let adapter_opts = wgpu::RequestAdapterOptions {
        //     power_preference: wgpu::PowerPreference::default(),
        //     compatible_surface: Some(&surface),
        //     force_fallback_adapter: false,
        // };

        // log::info!("Adapter");
        // let adapter = instance
        //     .request_adapter(&adapter_opts)
        //     .await
        //     .expect("Failed to find adapter");

        // // let limits = wgpu::Limits::default().using_resolution(adapter.limits());
        // let limits = wgpu::Limits {
        //     max_texture_dimension_1d: 4096,
        //     max_texture_dimension_2d: 4096,
        //     max_texture_dimension_3d: 256,
        //     max_texture_array_layers: 256,
        //     max_bind_groups: 4,
        //     max_dynamic_uniform_buffers_per_pipeline_layout: 8,
        //     // max_dynamic_storage_buffers_per_pipeline_layout: 4,
        //     max_sampled_textures_per_shader_stage: 16,
        //     max_samplers_per_shader_stage: 16,
        //     // max_storage_buffers_per_shader_stage: 4,
        //     // max_storage_textures_per_shader_stage: 4,
        //     max_uniform_buffers_per_shader_stage: 12,
        //     max_uniform_buffer_binding_size: 16384,
        //     // max_storage_buffer_binding_size: 128 << 20,
        //     max_vertex_buffers: 8,
        //     max_vertex_attributes: 16,
        //     // max_vertex_buffer_array_stride: 2048,
        //     max_push_constant_size: 0,
        //     min_uniform_buffer_offset_alignment: 256,
        //     min_storage_buffer_offset_alignment: 256,
        //     // These?
        //     max_storage_buffers_per_shader_stage: 0,
        //     max_storage_textures_per_shader_stage: 0,
        //     max_dynamic_storage_buffers_per_pipeline_layout: 0,
        //     max_storage_buffer_binding_size: 0,
        //     max_vertex_buffer_array_stride: 255,
        // };

        // log::info!("Device and Queue!");

        // let device_desc = wgpu::DeviceDescriptor {
        //     label: Some("conrod_device_descriptor"),
        //     features: wgpu::Features::empty(),
        //     limits,
        // };

        // // Create the logical device and command queue
        // let (device, mut queue) = adapter
        //     .request_device(&device_desc, None)
        //     .await
        //     .expect("Failed to create device");

        // // Create the swapchain.
        // log::info!("Get swapchain format");
        // let format = surface.get_preferred_format(&adapter).unwrap();

        let (format, (mut device, queue)) = futures::executor::block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .expect("Request adapter");

            (
                surface
                    .get_preferred_format(&adapter)
                    .expect("Get preferred format"),
                adapter
                    .request_device(
                        &wgpu::DeviceDescriptor {
                            label: None,
                            features: wgpu::Features::empty(),
                            limits: wgpu::Limits::default(),
                        },
                        None,
                    )
                    .await
                    .expect("Request device"),
            )
        });

        // let surface_config = wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format,
        //     width: size.width,
        //     height: size.height,
        //     present_mode: wgpu::PresentMode::Fifo,
        // };
        // surface.configure(&device, &surface_config);

        {
            let size = window.inner_size();

            surface.configure(
                &device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format,
                    width: size.width,
                    height: size.height,
                    present_mode: wgpu::PresentMode::Mailbox,
                },
            )
        };

        // Create the renderer for rendering conrod primitives.
        // let renderer = conrod_wgpu::Renderer::new(&device, MSAA_SAMPLES, format);

        // // The intermediary multisampled texture that will be resolved (MSAA).
        // let multisampled_framebuffer =
        //     create_multisampled_framebuffer(&device, &surface_config, MSAA_SAMPLES);

        // log::info!("Get image_map");
        // let mut image_map = conrod_core::image::Map::new();

        // eprint!("Generating UI\n");
        // let mut gui = gui;
        // let (win_size, scale_factor) = get_win_size(&window);
        // let ui = conrod_core::UiBuilder::new([win_size.width, win_size.height])
        //     .theme(gui.theme())
        //     .build();

        // let ui = gui.init(ui, &device, &mut queue, format, &mut image_map);

        let mut renderer = Renderer::new(Backend::new(&mut device, Settings::default(), format));

        Self {
            // size,
            // scale_factor,
            // surface,
            // device,
            // queue,
            // surface_config,
            // multisampled_framebuffer,
            // renderer,
            // image_map,
            // gui,
            // ui,
        }
    }

    pub fn gui(&mut self) {
        // Instantiate a GUI demonstrating every widget type provided by conrod.
        // conrod_example_shared::gui(&mut ui.set_widgets(), &ids, &mut app);
        // self.gui.gui(&mut self.ui.set_widgets());
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        log::info!("Resizing: {} x {}", new_size.width, new_size.height);

        // Recreate the swap chain with the new size
        // self.size = new_size;
        // self.surface_config.width = new_size.width;
        // self.surface_config.height = new_size.height;
        // self.surface.configure(&self.device, &self.surface_config);
        // self.multisampled_framebuffer =
        //     create_multisampled_framebuffer(&self.device, &self.surface_config, MSAA_SAMPLES);

        // self.render();
    }

    pub fn update(&mut self) {
        // self.gui()
    }

    pub fn ui_handle_event(&mut self, event: conrod_core::event::Input) {
        // println!("State::ui_handle_event: event = {:?}", event);
        // self.ui.handle_event(event);
    }

    pub fn ui_has_changed(&mut self) -> bool {
        // return self.ui.has_changed();
    }

    pub fn render(&mut self) -> Result<(), RenderError> {
        // // Feed the renderer primitives and update glyph cache texture if necessary.
        // let primitives = self.ui.draw();

        // // The window frame that we will draw to.
        // let surface_texture = self.surface.get_current_texture().unwrap();

        // // Begin encoding commands.
        // let cmd_encoder_desc = wgpu::CommandEncoderDescriptor {
        //     label: Some("conrod_command_encoder"),
        // };
        // let mut encoder = self.device.create_command_encoder(&cmd_encoder_desc);

        // let [win_w, win_h]: [f32; 2] = [self.size.width as f32, self.size.height as f32];
        // let viewport = [0.0, 0.0, win_w, win_h];
        // eprintln!("viewport: {:?}", viewport);

        // if let Some(cmd) = self
        //     .renderer
        //     .fill(&self.image_map, viewport, self.scale_factor, primitives)
        //     .unwrap()
        // {
        //     cmd.load_buffer_and_encode(&self.device, &mut encoder);
        // }

        // // Create a view for the surface's texture.
        // let surface_view = surface_texture
        //     .texture
        //     .create_view(&wgpu::TextureViewDescriptor::default());

        // // Begin the render pass and add the draw commands.
        // {
        //     // This condition allows to more easily tweak the MSAA_SAMPLES constant.
        //     let (attachment, resolve_target) = match MSAA_SAMPLES {
        //         1 => (&surface_view, None),
        //         _ => (&self.multisampled_framebuffer, Some(&surface_view)),
        //     };
        //     let color_attachment_desc = wgpu::RenderPassColorAttachment {
        //         view: attachment,
        //         resolve_target,
        //         ops: wgpu::Operations {
        //             load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
        //             store: true,
        //         },
        //     };

        //     let render_pass_desc = wgpu::RenderPassDescriptor {
        //         label: Some("conrod_render_pass_descriptor"),
        //         color_attachments: &[color_attachment_desc],
        //         depth_stencil_attachment: None,
        //     };
        //     let render = self.renderer.render(&self.device, &self.image_map);

        //     {
        //         let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
        //         let slot = 0;
        //         render_pass.set_vertex_buffer(slot, render.vertex_buffer.slice(..));
        //         let instance_range = 0..1;
        //         for cmd in render.commands {
        //             match cmd {
        //                 conrod_wgpu::RenderPassCommand::SetPipeline { pipeline } => {
        //                     render_pass.set_pipeline(pipeline);
        //                 }
        //                 conrod_wgpu::RenderPassCommand::SetBindGroup { bind_group } => {
        //                     render_pass.set_bind_group(0, bind_group, &[]);
        //                 }
        //                 conrod_wgpu::RenderPassCommand::SetScissor {
        //                     top_left,
        //                     dimensions,
        //                 } => {
        //                     let [x, y] = top_left;
        //                     let [w, h] = dimensions;
        //                     eprintln!("x = {}, y = {}, w = {}, h = {}", x, y, w, h);
        //                     render_pass.set_scissor_rect(x, y, w, h);
        //                 }
        //                 conrod_wgpu::RenderPassCommand::Draw { vertex_range } => {
        //                     render_pass.draw(vertex_range, instance_range.clone());
        //                 }
        //             }
        //         }
        //     }
        // }
        // self.queue.submit(Some(encoder.finish()));
        // surface_texture.present();

        Ok(())
    }
}
