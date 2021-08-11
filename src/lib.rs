pub fn entry() {
    #[cfg(not(target_os = "android"))]
    let _guard = sentry::init((
        "https://f885616c98c94558933f3d1ddc4d9cee@o925020.ingest.sentry.io/5873587",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
    // set assets
    utils::Utils::set_pc_assets_folder("assets");
    rsx_ui::App::new("main");
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
