use rsx_parser::types::RSXElement;

use crate::engine::dom::UiDom;

use super::super::engine::event::Event;
use super::super::engine::EventHandler;

use crate::engine::dom::view::View;

pub struct UiExample {
    ast: RSXElement,
}

impl UiExample {
    pub fn new(ast: RSXElement) -> Self {
        Self { ast }
    }
}

impl EventHandler for UiExample {
    // fn handle_event(&mut self, current_widget_id: u32, event: Event, cache: &mut WidgetCache) {
    //     eprintln!("Event received: {:?}", event);
    // }
    fn handle_event(&mut self, event: Event) {
        eprintln!("Event received: {:?}", event);
    }

    // fn build_layout(&mut self, cache: &mut WidgetCache) {
    fn build_layout(&mut self, dom: &mut UiDom) {
        eprintln!("Layout called.");

        let mut first_view = View::new();
        first_view.set_id("view1");
        first_view.set_origin(50, 50);
        first_view.set_dimensions(500, 400);
        first_view.set_background_color(0, 255, 0, 255);
        first_view.set_border_color(255, 0, 0, 255);
        first_view.set_border_width(10);

        let mut second_view = View::new();
        second_view.set_id("view2");
        second_view.set_origin(20, 20);
        second_view.set_dimensions(100, 100);
        second_view.set_background_color(0, 0, 255, 255);

        first_view.add(second_view);

        dom.add(first_view);

        // let mut base_widget = BaseWidget::default();

        // &base_widget.properties().set_origin(50, 50);
        // &base_widget.properties().set_bounds(540, 380);
        // &base_widget.properties().set_value(PROPERTY_BORDER_WIDTH, 2);
        // &base_widget
        //     .properties()
        //     .set_color(PROPERTY_BORDER_COLOR, Color::RGBA(0, 0, 0, 255));
        // &base_widget
        //     .properties()
        //     .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(0, 255, 0, 255));

        // let mut second_widget = BaseWidget::default();

        // &second_widget.properties().set_origin(60, 60);
        // &second_widget.properties().set_bounds(100, 200);
        // &second_widget
        //     .properties()
        //     .set_value(PROPERTY_BORDER_WIDTH, 10);
        // &second_widget
        //     .properties()
        //     .set_color(PROPERTY_BORDER_COLOR, Color::RGBA(255, 0, 0, 255));
        // &second_widget
        //     .properties()
        //     .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(0, 0, 255, 255));

        // cache.add(Box::new(base_widget), String::from("widget1"), 0);

        // cache.add(Box::new(second_widget), String::from("widget2"), 1);
    }
}
