use rsx_ui;

#[macroquad::main("BasicShapes")]
async fn main() {
    macroquad::file::set_pc_assets_folder("assets");
    rsx_ui::App::new("main").await;
}

/// Bindings entry point
#[no_mangle]
pub extern "C" fn main_rs() {
    main();
}
