use crate::{app_mode::AppMode, engine::Engine, rsx_lang, scripting_lang, ui::example::UiExample};

pub struct App {}

impl App {
    pub async fn new(entry_point: &str) {
        scripting_lang::Script::new(entry_point).await;
        let ast = rsx_lang::RSXLang::new(entry_point).await;
        let mut engine = Engine::new(Box::new(UiExample::new(ast)), AppMode::EDITOR);
        engine.run().await;
    }
}
