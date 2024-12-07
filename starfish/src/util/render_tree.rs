use std::{collections::HashMap, fmt::Debug};

use crate::shared::{
    render_backend::layout::{Layouter, LayoutTree},
    traits::{config::HasLayouter, render_tree},
    node::NodeId,
};

#[derive(Debug)]
pub struct RenderTree<C: HasLayouter> {
    pub nodes: HashMap<NodeId, RenderTreeNode<C>>,
}

#[allow(unused)]
impl<C: HasLayouter<LayoutTree = Self>> LayoutTree<C> for RenderTree<C> {

}

impl<C: HasLayouter<LayoutTree = Self>> render_tree::RenderTree<C> for RenderTree<C> {
    
}

impl<C: HasLayouter<LayoutTree = Self>> RenderTree<C> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut tree = Self {
            nodes: HashMap::with_capacity(capacity),
        };

        tree
    }
}

pub struct RenderTreeNode<C: HasLayouter> {
    pub layout: <C::Layouter as Layouter>::Layout,
}

impl<C: HasLayouter> Debug for RenderTreeNode<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderTreeNode")
            .finish()
    }
}
