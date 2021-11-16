//! A widget that allows for manually scrolling via dragging the mouse.

// use conrod_core::graph;
// use conrod_core::input::touch::Phase;
// use conrod_core::position::{Range, Scalar};
// use conrod_core::widget::scroll::{self, X, Y};

// use conrod_core::utils;
// use conrod_core::{self, widget, widget_ids, Positionable, Rect, Widget};

// use conrod_derive::WidgetCommon;
use std;

pub struct MoveEvent([f64; 2]);
impl MoveEvent {
    fn new() -> Self {
        Self([0.0, 0.0])
    }

    // fn set(&mut self, new_position: [f64; 2]) {
    //     self.0 = new_position
    // }

    // fn get(&self) -> [f64; 2] {
    //     self.0
    // }
}

/// A widget that allows for scrolling via dragging the mouse.
// #[derive(WidgetCommon)]
pub struct DraggableWidget<A> {
    // #[conrod(common_builder)]
    // common: widget::CommonBuilder,
    // widget_id: widget::Id,
    axis: std::marker::PhantomData<A>,
}

/// The axis that is scrolled by the `Scrollbar`.
// pub trait Axis: scroll::Axis + Sized {
//     /// The `Rect` for a scroll handle given both `Range`s.
//     fn handle_rect(perpendicular_track_range: Range, handle_range: Range) -> Rect;
//     /// Retrieve the related `scroll::State` for the axis from a given widget container.
//     fn scroll_state(widget: &graph::Container) -> Option<&scroll::State<Self>>;

//     /// Convert a given `Scalar` along the axis into two dimensions.
//     fn to_2d(scalar: Scalar) -> [Scalar; 2];
// }

// widget_ids! {
//   struct Ids {
//       inner,
//   }
// }

/// The state of the `Scrollbar`.
pub struct State {
    // _ids: Ids,
    _previous_move: MoveEvent,
}

impl<A> DraggableWidget<A> {
    // /// Begin building a new scrollbar widget.
    // pub fn new(widget_id: widget::Id) -> Self {
    //     eprintln!("DraggableWidget with widget ID: {:?}", widget_id);
    //     DraggableWidget {
    //         common: widget::CommonBuilder::default(),
    //         widget_id,
    //         axis: std::marker::PhantomData,
    //     }
    // }
}

// impl DraggableWidget<X> {
//     /// Begin building a new scrollbar widget that scrolls along the *X* axis along the range of
//     /// the scrollable widget at the given Id.
//     pub fn x_axis(widget_id: widget::Id) -> Self {
//         DraggableWidget::new(widget_id)
//             .align_middle_x_of(widget_id)
//             .align_bottom_of(widget_id)
//     }
// }

// impl DraggableWidget<Y> {
//     /// Begin building a new scrollbar widget that scrolls along the *Y* axis along the range of
//     /// the scrollable widget at the given Id.
//     pub fn y_axis(widget_id: widget::Id) -> Self {
//         DraggableWidget::new(widget_id)
//             .align_middle_y_of(widget_id)
//             .align_right_of(widget_id)
//     }
// }

// impl<A> Widget for DraggableWidget<A>
// where
//     A: Axis,
// {
//     type State = State;
//     type Style = ();
//     type Event = ();

//     fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
//         State {
//             _ids: Ids::new(id_gen),
//             _previous_move: MoveEvent::new(),
//         }
//     }

//     fn style(&self) -> Self::Style {}

//     fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
//         let widget::UpdateArgs {
//             id: _,
//             // state,
//             rect: _,
//             style: _,
//             ui,
//             ..
//         } = args;
//         let DraggableWidget {
//             widget_id: widget, ..
//         } = self;

//         // Sum all offset yielded by `Press` and `Drag` events.
//         let mut additional_offset: f64 = 0.0;
//         for widget_event in ui.widget_input(widget).events() {
//             use conrod_core::event;
//             // use conrod_core::input;

//             let handle_pos_range_len = || 99999.0;
//             let offset_bounds = || 99999.0;

//             match widget_event {
//                 // Check for the handle being dragged across the track.
//                 event::Widget::Drag(drag) => {
//                     let handle_pos_range_len = handle_pos_range_len();
//                     let offset_range_len = offset_bounds();
//                     let from_scalar = A::mouse_scalar(drag.from);
//                     let to_scalar = A::mouse_scalar(drag.to);
//                     let pos_offset = to_scalar - from_scalar;
//                     let offset = utils::map_range(
//                         pos_offset,
//                         0.0,
//                         handle_pos_range_len,
//                         0.0,
//                         offset_range_len,
//                     );
//                     eprintln!("DRAG: {:?} ", drag);
//                     additional_offset += offset;
//                 }

//                 event::Widget::Touch(touch) => {
//                     if touch.phase == Phase::Start {
//                         // state.previous_move.set(touch.xy);
//                         // TODO: state.update(f);
//                         // state.p
//                     }
//                     if touch.phase == Phase::Move {}
//                     let handle_pos_range_len = handle_pos_range_len();
//                     let offset_range_len = offset_bounds();
//                     let from_scalar = A::mouse_scalar(touch.xy);
//                     let to_scalar = A::mouse_scalar(touch.xy);
//                     let pos_offset = to_scalar - from_scalar;
//                     let offset = utils::map_range(
//                         pos_offset,
//                         0.0,
//                         handle_pos_range_len,
//                         0.0,
//                         offset_range_len,
//                     );
//                     eprintln!("TOUCH: {:?}", touch);
//                     additional_offset += offset;
//                 }

//                 _ => eprintln!("EVENT: {:?}", widget_event),
//             }
//         }

//         eprintln!("DRAG offset: {:?}", additional_offset);

//         // let mut drag_deltas = Vec::new();
//         // Ideally we'd use ui.widget_input() here, but any arbitrary child widget may have captured the mouse
//         // FIXME: This doesn't intercept the event, so the underlying widget might interpret it as well.

//         // for widget_event in ui.widget_input(id).events() {}

//         // for event in ui.global_input().events().ui() {
//         //     if let conrod_core::event::Ui::Drag(Some(drag_widget), drag) = event {
//         //         // Only capture events for the current widget and its children
//         //         if *drag_widget == id
//         //             || ui
//         //                 .widget_graph()
//         //                 .does_recursive_depth_edge_exist(id, *drag_widget)
//         //         {
//         //             drag_deltas.push(drag.delta_xy);
//         //         }
//         //     }
//         // }
//         // for drag_delta in drag_deltas {
//         //     eprintln!("Drag delta: {:?}", drag_delta);
//         //     ui.scroll_widget(self.widget_id, drag_delta);
//         // }

//         // Scroll the given widget by the accumulated additional offset.
//         if additional_offset != 0.0 && !additional_offset.is_nan() {
//             ui.scroll_widget(widget, A::to_2d(additional_offset));
//         }

//         // self.inner
//         //     .wh(rect.dim())
//         //     .xy(rect.xy())
//         //     .parent(id)
//         //     .set(state.ids.inner, ui)
//         // ui.scroll_widget(widget, A::to_2d(additional_offset));
//     }
// }

// impl Axis for X {
//     fn handle_rect(perpendicular_track_range: Range, handle_range: Range) -> Rect {
//         Rect {
//             x: handle_range,
//             y: perpendicular_track_range,
//         }
//     }

//     fn scroll_state(widget: &graph::Container) -> Option<&scroll::State<Self>> {
//         widget.maybe_x_scroll_state.as_ref()
//     }

//     fn to_2d(scalar: Scalar) -> [Scalar; 2] {
//         [scalar, 0.0]
//     }
// }

// impl Axis for Y {
//     fn handle_rect(perpendicular_track_range: Range, handle_range: Range) -> Rect {
//         Rect {
//             x: perpendicular_track_range,
//             y: handle_range,
//         }
//     }

//     fn scroll_state(widget: &graph::Container) -> Option<&scroll::State<Self>> {
//         widget.maybe_y_scroll_state.as_ref()
//     }

//     fn to_2d(scalar: Scalar) -> [Scalar; 2] {
//         [0.0, scalar]
//     }
// }
