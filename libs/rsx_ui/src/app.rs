use futures::executor::block_on;

use crate::{engine, rsx_lang, scripting_lang, ui::example::UiExample};

pub struct App {}

impl App {
    pub fn new(entry_point: &'static str) {
        // for now, scripts only show it can be loaded!
        block_on(App::start_script(entry_point));

        // 1. work on RSX syntax to create basic UI
        // 2. work on how RSX can be interpreted from script or rust_ui.rs file
        let ast = block_on(rsx_lang::RSXLang::new(entry_point));
        let app_mode = graphics::AppMode::APP;
        let ui = UiExample::new(ast);
        engine::Engine::run(app_mode, Box::new(ui));
    }

    async fn start_script(entry_point: &str) {
        scripting_lang::Script::new(entry_point).await;
    }
}
