
use crate::interface::layout::{Layout as TLayout, Layouter};

use taffy::Layout as TaffyLayout;

#[repr(transparent)]
#[derive(Default, Debug)]
pub struct Layout(TaffyLayout);

impl TLayout for Layout {

}

#[derive(Clone, Copy, Debug)]
pub struct TaffyLayouter;

impl Layouter for TaffyLayouter {
    type Layout = Layout;
}
