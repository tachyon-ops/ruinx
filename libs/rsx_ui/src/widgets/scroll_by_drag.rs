use conrod_core::{
    self, widget, widget_ids, Dimensions, Positionable, Rect, Sizeable, Theme, Widget,
};
use conrod_derive::WidgetCommon;

#[derive(WidgetCommon)]
pub struct ScrollByDrag<W: Widget + Sizeable + Positionable> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    inner: W,
}

widget_ids! {
  struct Ids {
      inner,
  }
}

pub struct State {
    ids: Ids,
}

impl<W: Widget + Sizeable + Positionable> ScrollByDrag<W> {
    pub fn new(inner: W) -> Self {
        ScrollByDrag {
            common: widget::CommonBuilder::default(),
            inner,
        }
    }
}

impl<W: Widget + Sizeable + Positionable> Widget for ScrollByDrag<W> {
    type State = State;
    type Style = ();
    type Event = W::Event;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {}

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            ui,
            rect,
            state,
            id,
            ..
        } = args;

        let mut drag_deltas = Vec::new();
        // Ideally we'd use ui.widget_input() here, but any arbitrary child widget may have captured the mouse
        // FIXME: This doesn't intercept the event, so the underlying widget might interpret it as well.
        for event in ui.global_input().events().ui() {
            if let conrod_core::event::Ui::Drag(Some(drag_widget), drag) = event {
                // Only capture events for the current widget and its children
                if *drag_widget == id
                    || ui
                        .widget_graph()
                        .does_recursive_depth_edge_exist(id, *drag_widget)
                {
                    drag_deltas.push(drag.delta_xy);
                }
            }
        }
        for drag_delta in drag_deltas {
            ui.scroll_widget(state.ids.inner, drag_delta);
        }

        self.inner
            .wh(rect.dim())
            .xy(rect.xy())
            .parent(id)
            .set(state.ids.inner, ui)
    }

    fn drag_area(&self, dim: Dimensions, _style: &Self::Style, _theme: &Theme) -> Option<Rect> {
        Some(Rect::from_xy_dim([0.0, 0.0], dim))
    }
}
