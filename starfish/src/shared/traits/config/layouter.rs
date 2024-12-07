use std::fmt::Debug;

use crate::shared::{
    render_backend::layout::{LayoutTree, Layouter},
    traits::config::HasCssSystem,
};

pub trait HasLayouter: HasCssSystem + Debug + 'static {
    type Layouter: Layouter;
    type LayoutTree: LayoutTree<Self>;
}
