pub mod dom;
pub mod event;
// pub mod widgets;

// Pushrod
use event::Event;

// Our own
use dom::UiDom;
use graphics::Stage;
use uuid::Uuid;

// SDL
// use sdl2::video::Window;
// use sdl2::Sdl;

pub trait EventHandler {
    /// This is the event handler that should be implemented when the `Event` handler is used.
    /// It provides the currently active widget ID and the event that was generated.
    /// Any events that could not be translated by `Pushrod` are either swallowed, or handled
    /// directly by the `run` method.  The cache is also provide as a way to get access to any
    /// `Widget`s in the list that need to be modified as the result of acting upon an `Event`.
    // fn handle_event(&mut self, current_view_id: u32, event: Event, cache: &mut WidgetCache);
    fn handle_event(&mut self, event: Event);

    /// This callback is used when the screen needs to be built for the first time.  It is called
    /// by the `Engine`'s `run` method before the event loop starts.  The `cache` is sent such that
    /// `Widget`s can be added to the display list by using the `WidgetCache`'s functions.
    // fn build_layout(&mut self, cache: &mut WidgetCache);
    fn build_layout(&mut self, dom: &mut UiDom);
}

pub struct Engine {
    current_view_uuid: Uuid,
    dom: UiDom,
    ui: Box<dyn EventHandler>,
    app_mode: graphics::AppMode,
}

impl Stage for Engine {
    fn get_mode(&mut self) -> graphics::AppMode {
        match self.app_mode {
            graphics::AppMode::GAME => graphics::AppMode::GAME,
            _ => graphics::AppMode::APP,
        }
    }
    fn setup(&mut self) {
        println!("Engine trait Stage will setup");
        self.ui.build_layout(&mut self.dom);
    }
    fn update(&mut self) {
        println!("Engine trait Stage will update");
    }
    fn render(&mut self) {
        println!("Engine trait State will draw");
    }
}

/// This is an implementation of `Pushrod`, the main loop handler.  Multiple `Pushrod`s
/// can be created for multiple windows if your application provides more than one window
/// with which to interact.
impl Engine {
    /// Creates a new `Pushrod` run loop, taking a reference to the `EventHandler` that handles
    /// run loop events for this `Window`.
    // pub fn new(handler: Box<dyn EventHandler>, window: &Window, editor_mode: bool) -> Self {
    pub fn run(app_mode: graphics::AppMode, ui: Box<dyn EventHandler>) {
        let size = (100, 100); // (screen_width() as u32, screen_height() as u32);
        let dom = UiDom::new(size);
        let engine = Self {
            current_view_uuid: dom.root.uuid,
            dom, // WidgetCache::new(window.size().0, window.size().1),
            ui,
            app_mode,
        };

        graphics::event_loop(Box::new(engine));
    }

    // / This is the main event handler for the application.  It handles all of the events generated
    // / by the `SDL2` manager, and translates them into events that can be used by the `handle_event`
    // / method.
    // pub fn run(&mut self, sdl: Sdl, window: Window) {
    // pub async fn _run(&self) {
    // // Call handler.build_layout() - this allows the application to build its `Window` contents,
    // // preparing the application for use.  (This is where the deserialization will occur.)

    // let index = macroquad::input::utils::register_input_subscriber();

    // let mut x = 100.0;
    // let mut y = 100.0;

    // self.handler.build_layout(&mut self.dom);

    // graphics::event_loop(stage);

    // loop {
    //     println!("___________________ LOOP ___________________");
    //     // clear_background(RED);

    //     // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    //     // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    //     // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    //     // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

    //     // for touch in touches() {
    //     //     let (fill_color, size) = match touch.phase {
    //     //         TouchPhase::Started => (GREEN, 80.0),
    //     //         TouchPhase::Stationary => (WHITE, 60.0),
    //     //         TouchPhase::Moved => (YELLOW, 60.0),
    //         TouchPhase::Ended => (BLUE, 80.0),
    //         TouchPhase::Cancelled => (BLACK, 80.0),
    //     };
    //     draw_circle(touch.position.x, touch.position.y, size, fill_color);
    // }

    // if is_key_down(KeyCode::Right) {
    //     x += 1.0;
    // }
    // if is_key_down(KeyCode::Left) {
    //     x -= 1.0;
    // }
    // if is_key_down(KeyCode::Down) {
    //     y += 1.0;
    // }
    // if is_key_down(KeyCode::Up) {
    //     y -= 1.0;
    // }

    // draw_circle(x, y, 15.0, YELLOW);

    // if self.app_mode == AppMode::EDITOR {
    //     // do event polling
    // }

    // macroquad::input::utils::repeat_all_miniquad_input(&mut stage, index);

    // // println!(
    // //     "condition touches().is_empty() || get_last_key_pressed().is_none() : {}",
    // //     touches().is_empty() || get_last_key_pressed().is_none()
    // // );
    // // while touches().is_empty() || get_last_key_pressed().is_none() {
    // //     println!("Inside condition");
    // //     // next_frame().await;
    // //     std::thread::sleep(std::time::Duration::new(0, 100));
    // // }

    // next_frame().await
    // }

    // 'running: loop {
    //     let start = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_millis();

    //     if self.editor_mode {
    //         let event = event_pump.wait_event();
    //         match event {
    //             sdl2::event::Event::Quit { .. } => break 'running,

    //             sdl2::event::Event::MouseMotion { x, y, .. } => {
    //                 let cur_widget_id = self.current_view_uuid;

    //                 self.current_view_uuid = self.dom.get_view_in_point(x as u32, y as u32);

    //                 if cur_widget_id != self.current_view_uuid {
    //                     // Send event to previous widget that the mouse has left scope
    //                     // Send event to current widget that the mouse has entered scope
    //                     eprintln!("Current Widget UUID: {}", self.current_view_uuid);
    //                 }
    //             }

    //             sdl2::event::Event::Window {
    //                 win_event,
    //                 timestamp,
    //                 window_id,
    //             } => match win_event {
    //                 sdl2::event::WindowEvent::Enter => {
    //                     eprintln!(
    //                         "Event: Window {{ timestamp: {}, window_id: {}, win_event: {:?} }}",
    //                         timestamp, window_id, win_event
    //                     );
    //                     eprintln!("Current Widget UUID: {}", self.current_view_uuid);
    //                 }
    //                 _ => eprintln!(
    //                     "Event: Window {{ timestamp: {}, window_id: {}, win_event: {:?} }}",
    //                     timestamp, window_id, win_event,
    //                 ),
    //             },

    //             unhandled_event => eprintln!("Event: {:?}", unhandled_event),
    //         }
    //     } else {
    //         // Process events first
    //         for event in event_pump.poll_iter() {
    //             match event {
    //                 sdl2::event::Event::Quit { .. } => break 'running,

    //                 sdl2::event::Event::MouseMotion { x, y, .. } => {
    //                     let cur_widget_id = self.current_view_uuid;

    //                     self.current_view_uuid = self.dom.get_view_in_point(x as u32, y as u32);

    //                     if cur_widget_id != self.current_view_uuid {
    //                         // Send event to previous widget that the mouse has left scope
    //                         // Send event to current widget that the mouse has entered scope
    //                         eprintln!("Current Widget UUID: {}", self.current_view_uuid);
    //                     }
    //                 }

    //                 sdl2::event::Event::Window {
    //                     win_event,
    //                     timestamp,
    //                     window_id,
    //                 } => match win_event {
    //                     sdl2::event::WindowEvent::Enter => {
    //                         eprintln!("Event: Window {{ timestamp: {}, window_id: {}, win_event: {:?} }}", timestamp, window_id, win_event);
    //                         eprintln!("Current Widget UUID: {}", self.current_view_uuid);
    //                     }
    //                     _ => eprintln!(
    //                         "Event: Window {{ timestamp: {}, window_id: {}, win_event: {:?} }}",
    //                         timestamp, window_id, win_event,
    //                     ),
    //                 },

    //                 unhandled_event => eprintln!("Event: {:?}", unhandled_event),
    //             }
    //         }
    //     }

    //     // Clear the canvas first.
    //     canvas.clear();
    //     // Draw after events are processed.
    //     self.dom.draw(&mut canvas);
    //     // Then swap the canvas once the draw is complete.
    //     canvas.present();

    //     let now = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_millis();

    //     if now - start < fps_as_ms {
    //         let diff = fps_as_ms - (now - start);
    //         if !self.editor_mode {
    //             sleep(Duration::from_millis(diff as u64));
    //         }
    //     }

    //     if !self.running {
    //         break 'running;
    //     }

    //     if first_run {
    //         first_run = false;
    //         canvas.window_mut().show();
    //     }
    // }
    // }
}
