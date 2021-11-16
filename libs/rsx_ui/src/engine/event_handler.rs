use super::dom::UiDom;

/// This is an event handler that is passed into a main event loop.  Since there can be multiple
/// windows open at any one time, the event handler that is implemented using this `trait` should
/// be for the window with which it is interacting.
///
/// It is inadvisable to create a single event handler "catch-all" for all application windows.
/// You will most likely get unexpected results.
pub trait EventHandler {
    /// This is the event handler that should be implemented when the `Event` handler is used.
    /// It provides the currently active widget ID and the event that was generated.
    /// Any events that could not be translated by `Pushrod` are either swallowed, or handled
    /// directly by the `run` method.  The cache is also provide as a way to get access to any
    /// `Widget`s in the list that need to be modified as the result of acting upon an `Event`.
    fn handle_event(
        &mut self,
        // current_widget_id: u32,
        event: graphics::events::Event,
        // cache: &mut WidgetCache,
    );

    /// This callback is used when the screen needs to be built for the first time.  It is called
    /// by the `Engine`'s `run` method before the event loop starts.  The `cache` is sent such that
    /// `Widget`s can be added to the display list by using the `WidgetCache`'s functions.
    // fn build_layout(&mut self, cache: &mut WidgetCache);
    fn build_layout(&mut self, dom: &mut UiDom);
}
