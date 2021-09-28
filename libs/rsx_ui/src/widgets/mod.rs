pub mod scroll_by_drag;

pub mod draggable_widget;
pub trait PositionableSizeableExt {
    fn fill(self, parent: conrod_core::widget::id::Id) -> Self;
}

impl<W: conrod_core::Positionable + conrod_core::Sizeable> PositionableSizeableExt for W {
    fn fill(self, parent: conrod_core::widget::id::Id) -> Self {
        self.kid_area_wh_of(parent).middle_of(parent)
    }
}
