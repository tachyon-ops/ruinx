use crate::{Application, RenderError};

use iced_wgpu::{
    wgpu::TextureFormat,
    wgpu::{self, Device, Queue, Surface},
    Backend, Renderer, Settings, Viewport,
};
use iced_winit::{conversion, program, winit, Clipboard, Debug, Size};

use iced_winit::futures::task::SpawnExt;

use winit::{dpi::PhysicalPosition, window::Window};

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

pub struct State<A>
where
    A: Application + 'static,
{
    cursor_position: PhysicalPosition<f64>,
    viewport: Viewport,
    resized: bool,
    state: program::State<A>,
    renderer: Renderer,
    clipboard: Clipboard,
    debug: Debug,
    surface: Surface,
    device: Device,
    format: TextureFormat,
    queue: Queue,
    staging_belt: iced_wgpu::wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
}

impl<A> State<A>
where
    A: Application + 'static,
{
    pub async fn new(window: &Window, application: A) -> Self {
        wait_for_native_window();

        let physical_size = window.inner_size();
        let viewport = Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            window.scale_factor(),
        );
        let cursor_position = PhysicalPosition::new(-1.0, -1.0);

        let clipboard = Clipboard::connect(&window);

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

        let (format, (mut device, queue)) = {
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
        };

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

        let mut renderer = Renderer::new(Backend::new(&mut device, Settings::default(), format));

        // Initialize scene and GUI controls
        // let scene = Scene::new(&mut device);
        // let program = Controls::new();
        let mut debug = Debug::new();

        let state = program::State::new(
            application,
            viewport.logical_size(),
            &mut renderer,
            &mut debug,
        );

        // Initialize staging belt and local pool
        let staging_belt = iced_wgpu::wgpu::util::StagingBelt::new(5 * 1024);
        let local_pool = futures::executor::LocalPool::new();

        Self {
            cursor_position,
            viewport,
            resized: false,
            state,
            renderer,
            clipboard,
            debug,
            surface,
            device,
            format,
            queue,
            staging_belt,
            local_pool,
        }
    }

    pub fn resize(&mut self, viewport: Viewport) {
        // log::info!("Resizing: {} x {}", new_size.width, new_size.height);
        self.viewport = viewport;
        self.resized = true;
    }

    pub fn render(&mut self, window: &Window) -> Result<(), RenderError> {
        if self.resized {
            let size = window.inner_size();

            self.surface.configure(
                &self.device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: self.format,
                    width: size.width,
                    height: size.height,
                    present_mode: wgpu::PresentMode::Mailbox,
                },
            );

            self.resized = false;
        }

        match self.surface.get_current_texture() {
            Ok(frame) => {
                let mut encoder = self
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                let program = self.state.program();

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                {
                    // We clear the frame
                    // let mut render_pass =
                    //     scene.clear(&view, &mut encoder, program.background_color());

                    // Draw the scene
                    // scene.draw(&mut render_pass);

                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear({
                                    let [r, g, b, a] = program.background_color().into_linear();

                                    wgpu::Color {
                                        r: r as f64,
                                        g: g as f64,
                                        b: b as f64,
                                        a: a as f64,
                                    }
                                }),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                }

                // And then iced on top
                let renderer = &mut self.renderer;
                let mut device = &mut self.device;
                let mut staging_belt = &mut self.staging_belt;
                let viewport = &mut self.viewport;
                let debug = &mut self.debug;
                renderer.with_primitives(|backend, primitive| {
                    backend.present(
                        &mut device,
                        &mut staging_belt,
                        &mut encoder,
                        &view,
                        primitive,
                        &viewport,
                        &debug.overlay(),
                    );
                });

                // Then we submit the work
                self.staging_belt.finish();
                self.queue.submit(Some(encoder.finish()));
                frame.present();

                // Update the mouse cursor
                window.set_cursor_icon(iced_winit::conversion::mouse_interaction(
                    self.state.mouse_interaction(),
                ));

                // And recall staging buffers
                self.local_pool
                    .spawner()
                    .spawn(self.staging_belt.recall())
                    .expect("Recall staging buffers");

                self.local_pool.run_until_stalled();
            }
            Err(error) => match error {
                wgpu::SurfaceError::OutOfMemory => {
                    panic!("Swapchain error: {}. Rendering cannot continue.", error)
                }
                _ => {
                    // Try rendering again next frame.
                    window.request_redraw();
                }
            },
        }

        Ok(())
    }

    pub fn set_cursor_position(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = position;
    }

    pub fn queue_event(&mut self, event: iced_winit::Event) {
        self.state.queue_event(event);
    }

    pub fn is_queue_empty(&mut self) -> bool {
        self.state.is_queue_empty()
    }

    pub fn update(&mut self) {
        let cursor_position =
            conversion::cursor_position(self.cursor_position, self.viewport.scale_factor());
        let _ = self.state.update(
            self.viewport.logical_size(),
            cursor_position,
            &mut self.renderer,
            &mut self.clipboard,
            &mut self.debug,
        );
    }
}
