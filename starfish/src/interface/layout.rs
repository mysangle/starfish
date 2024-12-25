use std::fmt::Debug;

use crate::interface::config::HasLayouter;

pub trait LayoutTree<C: HasLayouter<LayoutTree = Self>>: Sized + Debug + 'static {

}

pub trait Layouter: Sized + Clone + Send + 'static {
    type Layout: Layout + Send;
}

pub trait Layout: Default + Debug {
    
}
