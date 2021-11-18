mod controls;
pub mod dom;
mod event_handler;
// pub mod widgets;

// Our own
use dom::UiDom;
pub use event_handler::EventHandler;
use graphics::{winit, Engine};
use uuid::Uuid;

// use conrod_core::widget_ids;
// use wgpu::{Device, Queue};

// use crate::widgets::draggable_widget::DraggableWidget;

pub struct UiEngine {
    _current_view_uuid: Uuid,
    dom: UiDom,
    ui: Box<dyn EventHandler>,
    app_mode: graphics::AppMode,
}

impl Engine for UiEngine {
    fn get_mode(&mut self) -> graphics::AppMode {
        match self.app_mode {
            graphics::AppMode::GAME => graphics::AppMode::GAME,
            _ => graphics::AppMode::APP,
        }
    }
    fn update(&mut self) {
        // println!("Engine trait Stage will update");
    }
    fn render(&mut self) -> std::result::Result<(), graphics::RenderError> {
        // println!("Engine trait State will draw");
        Ok(())
    }

    fn setup(&mut self) {
        self.ui.build_layout(&mut self.dom);
    }

    fn event(&mut self, event: &winit::event::Event<()>) {
        self.ui.handle_event(graphics::events::Event::Wintit(event));
    }

    fn resize(&mut self, _size: winit::dpi::PhysicalSize<u32>) {
        // println!("Resize event triggered {}x{}", size.width, size.height);
    }
}

/// This is an implementation of `Pushrod`, the main loop handler.  Multiple `Pushrod`s
/// can be created for multiple windows if your application provides more than one window
/// with which to interact.
impl UiEngine {
    /// Creates a new `Pushrod` run loop, taking a reference to the `EventHandler` that handles
    /// run loop events for this `Window`.
    // pub fn new(handler: Box<dyn EventHandler>, window: &Window, editor_mode: bool) -> Self {
    pub fn run(name: &'static str, app_mode: graphics::AppMode, ui: Box<dyn EventHandler>) {
        let size = (100, 100); // (screen_width() as u32, screen_height() as u32);
        let dom = UiDom::new(size);

        // let engine = Self {
        let _ = Self {
            _current_view_uuid: dom.root.uuid,
            dom, // WidgetCache::new(window.size().0, window.size().1),
            ui,
            app_mode,
        };

        let application = controls::Controls::new();
        // graphics::event_loop(name, Box::new(engine), Box::new(DemoGui::new()));
        graphics::event_loop(name, application);
    }
}

// Generate a unique `WidgetId` for each widget.
// widget_ids! {
//   #[derive(Clone)]
//   pub struct Ids {
//     root,
//     // The scrollable canvas.
//     canvas,
//     // draggable
//     draggable_scroll,
//     // The title and introduction widgets.
//     title,
//     introduction,
//     // Shapes.
//     shapes_canvas,
//     rounded_rectangle,
//     shapes_left_col,
//     shapes_right_col,
//     shapes_title,
//     line,
//     point_path,
//     rectangle_fill,
//     rectangle_outline,
//     trapezoid,
//     oval_fill,
//     oval_outline,
//     circle,
//     // Image.
//     image_title,
//     rust_logo,
//     // Button, XyPad, Toggle.
//     button_title,
//     button,
//     xy_pad,
//     toggle,
//     ball,
//     // NumberDialer, PlotPath
//     dialer_title,
//     number_dialer,
//     plot_path,
//     // Scrollbar
//     canvas_scrollbar,

//     // new
//     login_page_user_list
//   }
// }

#[derive(Clone)]
pub struct DemoGui {
    // ids: Option<Ids>,
// ball_xy: conrod_core::Point,
// ball_color: conrod_core::Color,
// sine_frequency: f32,
// rust_logo: Option<conrod_core::image::Id>,
}

impl DemoGui {
    /// Simple constructor for the `DemoApp`.
    pub fn new() -> Self {
        DemoGui {
            // ids: None,
            // ball_xy: [0.0, 0.0],
            // ball_color: conrod_core::color::WHITE,
            // sine_frequency: 1.0,
            // rust_logo: None,
        }
    }
    // pub fn set_rust_logo(&mut self, rust_logo: conrod_core::image::Id) {
    //     self.rust_logo = Some(rust_logo);
    // }
}

impl graphics::GuiTrait for DemoGui {
    fn box_clone(&self) -> Box<(dyn graphics::GuiTrait + 'static)> {
        Box::new(self.clone())
    }

    // fn init(
    //     &mut self,
    //     mut ui: conrod_core::Ui,
    //     device: &Device,
    //     queue: &mut Queue,
    //     format: wgpu::TextureFormat,
    //     image_map: &mut conrod_core::image::Map<conrod_wgpu::Image>,
    // ) -> conrod_core::Ui {
    //     // Load font from file
    //     let font_path = "fonts/NotoSans/NotoSans-Regular.ttf";
    //     let font = crate::assets::load_font(font_path);
    //     ui.fonts.insert(font);

    //     // Load the Rust logo from our assets folder to use as an example image.F
    //     let logo = "images/rust.png";
    //     let rgba_logo_image = crate::assets::load_image(logo).to_rgba8();

    //     // Create the GPU texture and upload the image data.
    //     let (logo_w, logo_h) = rgba_logo_image.dimensions();
    //     eprintln!("Logo dimensions: {} x {}", logo_w, logo_h);
    //     let logo_tex = graphics::create_logo_texture(&device, queue, rgba_logo_image);

    //     let logo = conrod_wgpu::Image {
    //         texture: logo_tex,
    //         texture_format: format,
    //         width: logo_w,
    //         height: logo_h,
    //     };
    //     let rust_logo = image_map.insert(logo);
    //     self.rust_logo = Some(rust_logo);

    //     self.ids = Some(Ids::new(ui.widget_id_generator()));

    //     ui
    // }

    // /// Instantiate a GUI demonstrating every widget available in conrod.
    // fn gui(&mut self, ui: &mut conrod_core::UiCell) {
    //     if let Some(ids) = &mut self.ids {
    //         use conrod_core::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    //         use std::iter::once;

    //         const MARGIN: conrod_core::Scalar = 30.0;
    //         const SHAPE_GAP: conrod_core::Scalar = 50.0;
    //         const TITLE_SIZE: conrod_core::FontSize = 42;
    //         const SUBTITLE_SIZE: conrod_core::FontSize = 32;

    //         // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
    //         // By default, its size is the size of the window. We'll use this as a background for the
    //         // following widgets, as well as a scrollable container for the children widgets.
    //         const TITLE: &'static str = "All Widgets";
    //         widget::Canvas::new()
    //             .pad(MARGIN)
    //             .scroll_kids_vertically()
    //             .set(ids.canvas, ui);

    //         ////////////////
    //         ///// TEXT /////
    //         ////////////////

    //         // We'll demonstrate the `Text` primitive widget by using it to draw a title and an
    //         // introduction to the example.
    //         widget::Text::new(TITLE)
    //             .font_size(TITLE_SIZE)
    //             .mid_top_of(ids.canvas)
    //             .set(ids.title, ui);

    //         const INTRODUCTION: &'static str =
    //             "This example aims to demonstrate all widgets that are provided by conrod.\
    //      \n\nThe widget that you are currently looking at is the Text widget. The Text widget \
    //      is one of several special \"primitive\" widget types which are used to construct \
    //      all other widget types. These types are \"special\" in the sense that conrod knows \
    //      how to render them via `conrod_core::render::Primitive`s.\
    //      \n\nScroll down to see more widgets!";
    //         widget::Text::new(INTRODUCTION)
    //             .padded_w_of(ids.canvas, MARGIN)
    //             .down(60.0)
    //             .align_middle_x_of(ids.canvas)
    //             .center_justify()
    //             .line_spacing(5.0)
    //             .set(ids.introduction, ui);

    //         ////////////////////////////
    //         ///// Lines and Shapes /////
    //         ////////////////////////////

    //         widget::Text::new("Lines and Shapes")
    //             .down(70.0)
    //             .align_middle_x_of(ids.canvas)
    //             .font_size(SUBTITLE_SIZE)
    //             .set(ids.shapes_title, ui);

    //         // Lay out the shapes in two horizontal columns.
    //         //
    //         // TODO: Have conrod provide an auto-flowing, fluid-list widget that is more adaptive for these
    //         // sorts of situations.
    //         widget::Canvas::new()
    //             .down(0.0)
    //             .align_middle_x_of(ids.canvas)
    //             .kid_area_w_of(ids.canvas)
    //             .h(360.0)
    //             .color(conrod_core::color::TRANSPARENT)
    //             .pad(MARGIN)
    //             .flow_down(&[
    //                 (ids.shapes_left_col, widget::Canvas::new()),
    //                 (ids.shapes_right_col, widget::Canvas::new()),
    //             ])
    //             .set(ids.shapes_canvas, ui);

    //         let shapes_canvas_rect = ui.rect_of(ids.shapes_canvas).unwrap();
    //         let w = shapes_canvas_rect.w();
    //         let h = shapes_canvas_rect.h() * 5.0 / 6.0;
    //         let radius = 10.0;
    //         widget::RoundedRectangle::fill([w, h], radius)
    //             .color(conrod_core::color::CHARCOAL.alpha(0.25))
    //             .middle_of(ids.shapes_canvas)
    //             .set(ids.rounded_rectangle, ui);

    //         let start = [-40.0, -40.0];
    //         let end = [40.0, 40.0];
    //         widget::Line::centred(start, end)
    //             .mid_left_of(ids.shapes_left_col)
    //             .set(ids.line, ui);

    //         let left = [-40.0, -40.0];
    //         let top = [0.0, 40.0];
    //         let right = [40.0, -40.0];
    //         let points = once(left).chain(once(top)).chain(once(right));
    //         widget::PointPath::centred(points)
    //             .right(SHAPE_GAP)
    //             .set(ids.point_path, ui);

    //         widget::Rectangle::fill([80.0, 80.0])
    //             .right(SHAPE_GAP)
    //             .set(ids.rectangle_fill, ui);

    //         widget::Rectangle::outline([80.0, 80.0])
    //             .right(SHAPE_GAP)
    //             .set(ids.rectangle_outline, ui);

    //         let bl = [-40.0, -40.0];
    //         let tl = [-20.0, 40.0];
    //         let tr = [20.0, 40.0];
    //         let br = [40.0, -40.0];
    //         let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
    //         widget::Polygon::centred_fill(points)
    //             .mid_left_of(ids.shapes_right_col)
    //             .set(ids.trapezoid, ui);

    //         widget::Oval::fill([40.0, 80.0])
    //             .right(SHAPE_GAP + 20.0)
    //             .align_middle_y()
    //             .set(ids.oval_fill, ui);

    //         widget::Oval::outline([80.0, 40.0])
    //             .right(SHAPE_GAP + 20.0)
    //             .align_middle_y()
    //             .set(ids.oval_outline, ui);

    //         widget::Circle::fill(40.0)
    //             .right(SHAPE_GAP)
    //             .align_middle_y()
    //             .set(ids.circle, ui);

    //         /////////////////
    //         ///// Image /////
    //         /////////////////

    //         if let Some(logo) = self.rust_logo {
    //             widget::Text::new("Image")
    //                 .down_from(ids.shapes_canvas, MARGIN)
    //                 .align_middle_x_of(ids.canvas)
    //                 .font_size(SUBTITLE_SIZE)
    //                 .set(ids.image_title, ui);

    //             const LOGO_SIDE: conrod_core::Scalar = 144.0;
    //             widget::Image::new(logo)
    //                 .w_h(LOGO_SIDE, LOGO_SIDE)
    //                 .down(60.0)
    //                 .align_middle_x_of(ids.canvas)
    //                 .set(ids.rust_logo, ui);
    //         }

    //         /////////////////////////////////
    //         ///// Button, XYPad, Toggle /////
    //         /////////////////////////////////

    //         widget::Text::new("Button, XYPad and Toggle")
    //             .down_from(ids.rust_logo, 60.0)
    //             .align_middle_x_of(ids.canvas)
    //             .font_size(SUBTITLE_SIZE)
    //             .set(ids.button_title, ui);

    //         let ball_x_range = ui.kid_area_of(ids.canvas).unwrap().w();
    //         let ball_y_range = ui.h_of(ui.window).unwrap() * 0.5;
    //         let min_x = -ball_x_range / 3.0;
    //         let max_x = ball_x_range / 3.0;
    //         let min_y = -ball_y_range / 3.0;
    //         let max_y = ball_y_range / 3.0;
    //         let side = 130.0;

    //         for _press in widget::Button::new()
    //             .label("PRESS ME")
    //             .mid_left_with_margin_on(ids.canvas, MARGIN)
    //             .down_from(ids.button_title, 60.0)
    //             .w_h(side, side)
    //             .set(ids.button, ui)
    //         {
    //             let x = rand::random::<conrod_core::Scalar>() * (max_x - min_x) - max_x;
    //             let y = rand::random::<conrod_core::Scalar>() * (max_y - min_y) - max_y;
    //             self.ball_xy = [x, y];
    //         }

    //         for (x, y) in
    //             widget::XYPad::new(self.ball_xy[0], min_x, max_x, self.ball_xy[1], min_y, max_y)
    //                 .label("BALL XY")
    //                 .wh_of(ids.button)
    //                 .align_middle_y_of(ids.button)
    //                 .align_middle_x_of(ids.canvas)
    //                 .parent(ids.canvas)
    //                 .set(ids.xy_pad, ui)
    //         {
    //             self.ball_xy = [x, y];
    //         }

    //         let is_white = self.ball_color == conrod_core::color::WHITE;
    //         let label = if is_white { "WHITE" } else { "BLACK" };
    //         for is_white in widget::Toggle::new(is_white)
    //             .label(label)
    //             .label_color(if is_white {
    //                 conrod_core::color::WHITE
    //             } else {
    //                 conrod_core::color::LIGHT_CHARCOAL
    //             })
    //             .mid_right_with_margin_on(ids.canvas, MARGIN)
    //             .align_middle_y_of(ids.button)
    //             .set(ids.toggle, ui)
    //         {
    //             self.ball_color = if is_white {
    //                 conrod_core::color::WHITE
    //             } else {
    //                 conrod_core::color::BLACK
    //             };
    //         }

    //         let ball_x = self.ball_xy[0];
    //         let ball_y = self.ball_xy[1] - max_y - side * 0.5 - MARGIN;
    //         widget::Circle::fill(20.0)
    //             .color(self.ball_color)
    //             .x_y_relative_to(ids.xy_pad, ball_x, ball_y)
    //             .set(ids.ball, ui);

    //         //////////////////////////////////
    //         ///// NumberDialer, PlotPath /////
    //         //////////////////////////////////

    //         widget::Text::new("NumberDialer and PlotPath")
    //             .down_from(ids.xy_pad, max_y - min_y + side * 0.5 + MARGIN)
    //             .align_middle_x_of(ids.canvas)
    //             .font_size(SUBTITLE_SIZE)
    //             .set(ids.dialer_title, ui);

    //         // Use a `NumberDialer` widget to adjust the frequency of the sine wave below.
    //         let min = 0.5;
    //         let max = 200.0;
    //         let decimal_precision = 1;
    //         for new_freq in
    //             widget::NumberDialer::new(self.sine_frequency, min, max, decimal_precision)
    //                 .down(60.0)
    //                 .align_middle_x_of(ids.canvas)
    //                 .w_h(160.0, 40.0)
    //                 .label("F R E Q")
    //                 .set(ids.number_dialer, ui)
    //         {
    //             self.sine_frequency = new_freq;
    //         }

    //         // Use the `PlotPath` widget to display a sine wave.
    //         let min_x = 0.0;
    //         let max_x = std::f32::consts::PI * 2.0 * self.sine_frequency;
    //         let min_y = -1.0;
    //         let max_y = 1.0;
    //         widget::PlotPath::new(min_x, max_x, min_y, max_y, f32::sin)
    //             .kid_area_w_of(ids.canvas)
    //             .h(240.0)
    //             .down(60.0)
    //             .align_middle_x_of(ids.canvas)
    //             .set(ids.plot_path, ui);

    //         /////////////////////
    //         ///// Scrollbar /////
    //         /////////////////////

    //         widget::Scrollbar::y_axis(ids.canvas)
    //             .auto_hide(true)
    //             .set(ids.canvas_scrollbar, ui);

    //         // use conrod_core::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    //         // use std::iter::once;

    //         // const MARGIN: conrod_core::Scalar = 30.0;

    //         // // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
    //         // // By default, its size is the size of the window. We'll use this as a background for the
    //         // // following widgets, as well as a scrollable container for the children widgets.
    //         // widget::Canvas::new()
    //         //     .pad(MARGIN)
    //         //     .scroll_kids_vertically()
    //         //     // .border(10.0)
    //         //     // .border_rgba(0.0, 255.0, 0.0, 1.0)
    //         //     .set(ids.root, ui);

    //         // // let (mut items, scrollbar) = ScrollByDrag::new(
    //         // //     widget::List::flow_down(1), //.scrollbar_on_top(), // .item_size(30.0),
    //         // // )
    //         // // .fill(ids.root)
    //         // // .set(ids.draggable_scroll, ui);

    //         // // ScrollByDrag::new(inner)
    //         // //     // .kid_area_w_of(ids.root)
    //         // //     // .kid_area_h_of(ids.root)
    //         // //     // .scroll_kids_horizontally()
    //         // //     .fill(ids.root)
    //         // //     .set(ids.draggable_scroll, ui);

    //         // // while let Some(item) = items.next(ui) {
    //         // //     let inner = widget::Canvas::new()
    //         // //         // .pad(MARGIN)
    //         // //         .kid_area_w_of(ids.root)
    //         // //         .kid_area_h_of(ids.root)
    //         // //         .scroll_kids_vertically();
    //         // //     item.set(inner, ui);
    //         // //     inner.set(ids.canvas, ui);
    //         // // }

    //         // // // inner.set(ids.canvas, ui);
    //         // // if let Some(scrollbar) = scrollbar {
    //         // //     scrollbar.set(ui);
    //         // // }

    //         // widget::Canvas::new()
    //         //     // .pad(MARGIN)
    //         //     .kid_area_w_of(ids.root)
    //         //     .kid_area_h_of(ids.root)
    //         //     .scroll_kids_vertically()
    //         //     .set(ids.canvas, ui);

    //         // ////////////////
    //         // ///// TEXT /////
    //         // ////////////////

    //         // // We'll demonstrate the `Text` primitive widget by using it to draw a title and an
    //         // // introduction to the example.
    //         // const TITLE: &'static str = "RuinX";
    //         // const TITLE_SIZE: conrod_core::FontSize = 42;
    //         // widget::Text::new(TITLE)
    //         //     .font_size(TITLE_SIZE)
    //         //     .mid_top_of(ids.canvas)
    //         //     .set(ids.title, ui);

    //         // const INTRODUCTION: &'static str =
    //         //     "This example aims to demonstrate all widgets that are provided by RuinX through conrod.\
    //         //     \n\nThe widget that you are currently looking at is the Text widget. The Text widget \
    //         //     is one of several special \"primitive\" widget types which are used to construct \
    //         //     all other widget types. These types are \"special\" in the sense that conrod knows \
    //         //     how to render them via `conrod_core::render::Primitive`s.\
    //         //  \n\nScroll down to see more widgets!";
    //         // widget::Text::new(INTRODUCTION)
    //         //     .padded_w_of(ids.canvas, MARGIN)
    //         //     .down(60.0)
    //         //     .align_middle_x_of(ids.canvas)
    //         //     .center_justify()
    //         //     .line_spacing(5.0)
    //         //     .set(ids.introduction, ui);

    //         // ////////////////////////////
    //         // ///// Lines and Shapes /////
    //         // ////////////////////////////

    //         // const SUBTITLE_SIZE: conrod_core::FontSize = 32;
    //         // widget::Text::new("Lines and Shapes")
    //         //     .down(70.0)
    //         //     .align_middle_x_of(ids.canvas)
    //         //     .font_size(SUBTITLE_SIZE)
    //         //     .set(ids.shapes_title, ui);

    //         // // Lay out the shapes in two horizontal columns.
    //         // //
    //         // // TODO: Have conrod provide an auto-flowing, fluid-list widget that is more adaptive for these
    //         // // sorts of situations.
    //         // widget::Canvas::new()
    //         //     .down(0.0)
    //         //     .align_middle_x_of(ids.canvas)
    //         //     .kid_area_w_of(ids.canvas)
    //         //     .h(360.0)
    //         //     .color(conrod_core::color::TRANSPARENT)
    //         //     .pad(MARGIN)
    //         //     .flow_down(&[
    //         //         (ids.shapes_left_col, widget::Canvas::new()),
    //         //         (ids.shapes_right_col, widget::Canvas::new()),
    //         //     ])
    //         //     .set(ids.shapes_canvas, ui);

    //         // let shapes_canvas_rect = ui.rect_of(ids.shapes_canvas).unwrap();
    //         // let w = shapes_canvas_rect.w();
    //         // let h = shapes_canvas_rect.h() * 5.0 / 6.0;
    //         // let radius = 10.0;
    //         // widget::RoundedRectangle::fill([w, h], radius)
    //         //     .color(conrod_core::color::CHARCOAL.alpha(0.25))
    //         //     .middle_of(ids.shapes_canvas)
    //         //     .set(ids.rounded_rectangle, ui);

    //         // let start = [-40.0, -40.0];
    //         // let end = [40.0, 40.0];
    //         // widget::Line::centred(start, end)
    //         //     .mid_left_of(ids.shapes_left_col)
    //         //     .set(ids.line, ui);

    //         // const SHAPE_GAP: conrod_core::Scalar = 50.0;

    //         // let left = [-40.0, -40.0];
    //         // let top = [0.0, 40.0];
    //         // let right = [40.0, -40.0];
    //         // let points = once(left).chain(once(top)).chain(once(right));
    //         // widget::PointPath::centred(points)
    //         //     .right(SHAPE_GAP)
    //         //     .set(ids.point_path, ui);

    //         // widget::Rectangle::fill([80.0, 80.0])
    //         //     .right(SHAPE_GAP)
    //         //     .set(ids.rectangle_fill, ui);

    //         // widget::Rectangle::outline([80.0, 80.0])
    //         //     .right(SHAPE_GAP)
    //         //     .set(ids.rectangle_outline, ui);

    //         // let bl = [-40.0, -40.0];
    //         // let tl = [-20.0, 40.0];
    //         // let tr = [20.0, 40.0];
    //         // let br = [40.0, -40.0];
    //         // let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
    //         // widget::Polygon::centred_fill(points)
    //         //     .mid_left_of(ids.shapes_right_col)
    //         //     .set(ids.trapezoid, ui);

    //         // widget::Oval::fill([40.0, 80.0])
    //         //     .right(SHAPE_GAP + 20.0)
    //         //     .align_middle_y()
    //         //     .set(ids.oval_fill, ui);

    //         // widget::Oval::outline([80.0, 40.0])
    //         //     .right(SHAPE_GAP + 20.0)
    //         //     .align_middle_y()
    //         //     .set(ids.oval_outline, ui);

    //         // widget::Circle::fill(40.0)
    //         //     .right(SHAPE_GAP)
    //         //     .align_middle_y()
    //         //     .set(ids.circle, ui);

    //         // /////////////////
    //         // ///// Image /////
    //         // /////////////////

    //         // if let Some(logo) = self.rust_logo {
    //         //     widget::Text::new("Image")
    //         //         .down_from(ids.shapes_canvas, MARGIN)
    //         //         .align_middle_x_of(ids.canvas)
    //         //         .font_size(SUBTITLE_SIZE)
    //         //         .set(ids.image_title, ui);

    //         //     const LOGO_SIDE: conrod_core::Scalar = 144.0;
    //         //     widget::Image::new(logo)
    //         //         .w_h(LOGO_SIDE, LOGO_SIDE)
    //         //         .down(60.0)
    //         //         .align_middle_x_of(ids.canvas)
    //         //         .set(ids.rust_logo, ui);
    //         // }

    //         // /////////////////////////////////
    //         // ///// Button, XYPad, Toggle /////
    //         // /////////////////////////////////

    //         // widget::Text::new("Button, XYPad and Toggle")
    //         //     .down_from(ids.rust_logo, 60.0)
    //         //     .align_middle_x_of(ids.canvas)
    //         //     .font_size(SUBTITLE_SIZE)
    //         //     .set(ids.button_title, ui);

    //         // let ball_x_range = ui.kid_area_of(ids.canvas).unwrap().w();
    //         // let ball_y_range = ui.h_of(ui.window).unwrap() * 0.5;
    //         // let min_x = -ball_x_range / 3.0;
    //         // let max_x = ball_x_range / 3.0;
    //         // let min_y = -ball_y_range / 3.0;
    //         // let max_y = ball_y_range / 3.0;
    //         // let side = 130.0;

    //         // for _press in widget::Button::new()
    //         //     .label("PRESS ME")
    //         //     .mid_left_with_margin_on(ids.canvas, MARGIN)
    //         //     .down_from(ids.button_title, 60.0)
    //         //     .w_h(side, side)
    //         //     .set(ids.button, ui)
    //         // {
    //         //     let x = rand::random::<conrod_core::Scalar>() * (max_x - min_x) - max_x;
    //         //     let y = rand::random::<conrod_core::Scalar>() * (max_y - min_y) - max_y;
    //         //     self.ball_xy = [x, y];
    //         // }

    //         // for (x, y) in
    //         //     widget::XYPad::new(self.ball_xy[0], min_x, max_x, self.ball_xy[1], min_y, max_y)
    //         //         .label("BALL XY")
    //         //         .wh_of(ids.button)
    //         //         .align_middle_y_of(ids.button)
    //         //         .align_middle_x_of(ids.canvas)
    //         //         .parent(ids.canvas)
    //         //         .set(ids.xy_pad, ui)
    //         // {
    //         //     self.ball_xy = [x, y];
    //         // }

    //         // let is_white = self.ball_color == conrod_core::color::WHITE;
    //         // let label = if is_white { "WHITE" } else { "BLACK" };
    //         // for is_white in widget::Toggle::new(is_white)
    //         //     .label(label)
    //         //     .label_color(if is_white {
    //         //         conrod_core::color::WHITE
    //         //     } else {
    //         //         conrod_core::color::LIGHT_CHARCOAL
    //         //     })
    //         //     .mid_right_with_margin_on(ids.canvas, MARGIN)
    //         //     .align_middle_y_of(ids.button)
    //         //     .set(ids.toggle, ui)
    //         // {
    //         //     self.ball_color = if is_white {
    //         //         conrod_core::color::WHITE
    //         //     } else {
    //         //         conrod_core::color::BLACK
    //         //     };
    //         // }

    //         // let ball_x = self.ball_xy[0];
    //         // let ball_y = self.ball_xy[1] - max_y - side * 0.5 - MARGIN;
    //         // widget::Circle::fill(20.0)
    //         //     .color(self.ball_color)
    //         //     .x_y_relative_to(ids.xy_pad, ball_x, ball_y)
    //         //     .set(ids.ball, ui);

    //         // //////////////////////////////////
    //         // ///// NumberDialer, PlotPath /////
    //         // //////////////////////////////////

    //         // widget::Text::new("NumberDialer and PlotPath")
    //         //     .down_from(ids.xy_pad, max_y - min_y + side * 0.5 + MARGIN)
    //         //     .align_middle_x_of(ids.canvas)
    //         //     .font_size(SUBTITLE_SIZE)
    //         //     .set(ids.dialer_title, ui);

    //         // // Use a `NumberDialer` widget to adjust the frequency of the sine wave below.
    //         // let min = 0.5;
    //         // let max = 200.0;
    //         // let decimal_precision = 1;
    //         // for new_freq in
    //         //     widget::NumberDialer::new(self.sine_frequency, min, max, decimal_precision)
    //         //         .down(60.0)
    //         //         .align_middle_x_of(ids.canvas)
    //         //         .w_h(160.0, 40.0)
    //         //         .label("F R E Q")
    //         //         .set(ids.number_dialer, ui)
    //         // {
    //         //     self.sine_frequency = new_freq;
    //         // }

    //         // // Use the `PlotPath` widget to display a sine wave.
    //         // let min_x = 0.0;
    //         // let max_x = std::f32::consts::PI * 2.0 * self.sine_frequency;
    //         // let min_y = -1.0;
    //         // let max_y = 1.0;
    //         // widget::PlotPath::new(min_x, max_x, min_y, max_y, f32::sin)
    //         //     .kid_area_w_of(ids.canvas)
    //         //     .h(240.0)
    //         //     .down(60.0)
    //         //     .align_middle_x_of(ids.canvas)
    //         //     .set(ids.plot_path, ui);

    //         // /////////////////////
    //         // ///// Scrollbar /////
    //         // /////////////////////

    //         // widget::Scrollbar::y_axis(ids.canvas)
    //         //     .auto_hide(false)
    //         //     .set(ids.canvas_scrollbar, ui);

    //         // DraggableWidget::y_axis(ids.canvas).set(ids.draggable_scroll, ui);
    //     }
    // }

    // /// A set of reasonable stylistic defaults that works for the `gui` below.
    // fn theme(&mut self) -> conrod_core::Theme {
    //     use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    //     conrod_core::Theme {
    //         name: "Demo Theme".to_string(),
    //         padding: Padding::none(),
    //         x_position: Position::Relative(Relative::Align(Align::Start), None),
    //         y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
    //         background_color: conrod_core::color::DARK_CHARCOAL,
    //         shape_color: conrod_core::color::LIGHT_CHARCOAL,
    //         border_color: conrod_core::color::BLACK,
    //         border_width: 0.0,
    //         label_color: conrod_core::color::WHITE,
    //         font_id: None,
    //         font_size_large: 26,
    //         font_size_medium: 18,
    //         font_size_small: 12,
    //         widget_styling: conrod_core::theme::StyleMap::default(),
    //         mouse_drag_threshold: 0.0,
    //         double_click_threshold: std::time::Duration::from_millis(500),
    //     }
    // }
}
