use super::engine::Engine;

mod example;
use example::UiExample;
use rsx_parser::types::RSXElement;

pub fn run(ast: RSXElement) -> Result<(), String> {
    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem
    //     .window("UI example", 640, 480)
    //     .position_centered()
    //     .hidden()
    //     .build()
    //     .unwrap();
    // let mut engine = Engine::new(Box::new(UiExample::new(ast)), &window, false);

    // engine.run(sdl_context, window);

    Ok(())
}
