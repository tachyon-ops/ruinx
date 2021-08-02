use rsx_ui;

#[macroquad::main("BasicShapes")]
async fn main() {
    // This fixes iOS path while macroquad doesn't merge and include PR
    #[cfg(target_os = "ios")]
    let _ = std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap());
    macroquad::file::set_pc_assets_folder("assets");
    rsx_ui::App::new("main").await;
}

/// Bindings entry point
#[no_mangle]
pub extern "C" fn main_rs() {
    main();
}
