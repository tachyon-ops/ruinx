use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

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
    log::info!("Proceeding after native window");
}

fn block_until_has_size(window: &Window) -> PhysicalSize<u32> {
    let mut size = window.inner_size();

    let mut condition = true;
    let mut i = 0;
    while condition {
        std::thread::sleep(std::time::Duration::from_millis(250));
        // wait_for_native_window();
        size = window.inner_size();
        log::info!("Window size is {} x {}", size.width, size.height);

        if size.width != 0 {
            condition = false;
        }
        if i >= 20 {
            condition = false;
        }
        i = i + 1;
    }
    log::info!("Window size is {} x {}", size.width, size.height);
    size
}

fn initialize_logger() {
    #[cfg(target_os = "android")]
    {
        let _trace;
        if ndk::trace::is_trace_enabled() {
            _trace = ndk::trace::Section::new("ndk-rs example main").unwrap();
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }

    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
}

pub fn entry() {
    wait_for_native_window();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    initialize_logger();

    #[cfg(not(target_arch = "wasm32"))]
    super::run(event_loop, window);

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}
