pub mod dom;
pub mod event;
// pub mod widgets;

// Pushrod
use event::Event;
// use widgets::WidgetCache;

// Our own
use dom::UiDom;
use uuid::Uuid;

// System
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    handler: Box<dyn EventHandler>,
    dom: UiDom,
    running: bool,
    editor_mode: bool,
}

/// This is an implementation of `Pushrod`, the main loop handler.  Multiple `Pushrod`s
/// can be created for multiple windows if your application provides more than one window
/// with which to interact.
impl Engine {
    /// Creates a new `Pushrod` run loop, taking a reference to the `EventHandler` that handles
    /// run loop events for this `Window`.
    // pub fn new(handler: Box<dyn EventHandler>, window: &Window, editor_mode: bool) -> Self {
    pub fn new(handler: Box<dyn EventHandler>, editor_mode: bool) -> Self {
        todo!();
        // let dom = UiDom::new(window.size());
        // Self {
        //     current_view_uuid: dom.root.uuid,
        //     handler,
        //     dom, // WidgetCache::new(window.size().0, window.size().1),
        //     running: true,
        //     editor_mode,
        // }
    }

    /// Stops the Pushrod run loop.
    // pub fn stop(&mut self) {
    //     self.running = false;
    // }

    // /// Retrieves the `WidgetCache`.
    // pub fn get_cache(&mut self) -> &mut WidgetCache {
    //     &mut self.cache
    // }

    /// This is the main event handler for the application.  It handles all of the events generated
    /// by the `SDL2` manager, and translates them into events that can be used by the `handle_event`
    /// method.
    // pub fn run(&mut self, sdl: Sdl, window: Window) {
    pub fn run(&mut self) {
        // let mut event_pump = sdl.event_pump().unwrap();
        // let fps_as_ms = (1000.0 / 60_f64) as u128;
        // let mut canvas = window
        //     .into_canvas()
        //     .target_texture()
        //     .accelerated()
        //     .build()
        //     .unwrap();

        // // Call handler.build_layout() - this allows the application to build its `Window` contents,
        // // preparing the application for use.  (This is where the deserialization will occur.)
        // self.handler.build_layout(&mut self.dom);

        // let mut first_run = true;

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
    }
}
