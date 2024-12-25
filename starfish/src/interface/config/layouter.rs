use std::fmt::Debug;

use crate::interface::{
    config::HasCssSystem,
    layout::{LayoutTree, Layouter},
};

pub trait HasLayouter: HasCssSystem + Debug + 'static {
    type Layouter: Layouter;
    type LayoutTree: LayoutTree<Self>;
}
