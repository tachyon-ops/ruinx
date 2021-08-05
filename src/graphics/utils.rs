use winit::{dpi::PhysicalSize, window::Window};
pub struct GraphicUtils;

impl GraphicUtils {
    pub fn wait_for_window() {
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

    pub fn wait_for_size(window: &Window) -> PhysicalSize<u32> {
        let mut size = window.inner_size();
        let mut i = 0;
        let mut condition = true;
        while condition {
            std::thread::sleep(std::time::Duration::from_millis(250));
            size = window.inner_size();
            log::info!(
                "Window size is {} x {}, iteration: {}",
                size.width,
                size.height,
                i
            );

            if size.width != 0 {
                condition = false;
            }
            if i >= 50 {
                condition = false;
            }
            i = i + 1;
        }

        log::info!("Window size is {} x {}", size.width, size.height);
        size
    }

    pub fn initialize_logger() {
        #[cfg(target_os = "android")]
        {
            eprintln!("Initializing logger for android");
            let _trace;
            if ndk::trace::is_trace_enabled() {
                _trace = ndk::trace::Section::new("ndk-rs example main").unwrap();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            eprintln!("Initializing logger for wasm32");
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
        }

        #[cfg(not(target_os = "android"))]
        #[cfg(not(target_arch = "wasm32"))]
        {
            eprintln!("Initializing logger for ios, mac, windows and linux");
            env_logger::init();
        }
    }
}
