use std::{collections::HashMap, fmt::Debug};

use crate::{
    interface::{
        config::{HasDocument, HasLayouter, HasRenderTree},
        document::Document,
        layout::{Layouter, LayoutTree},
        render_tree,
    },
    shared::{
        document::DocumentHandle,
        node::NodeId,
        types::Result,
    },
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

impl<C: HasRenderTree<LayoutTree = Self, RenderTree = Self> + HasDocument> RenderTree<C> {
    pub fn from_document(document: DocumentHandle<C>) -> Self {
        let mut render_tree = RenderTree::with_capacity(document.get().node_count());

        render_tree.generate_from(document);

        render_tree
    }

    fn generate_from(&mut self, mut handle: DocumentHandle<C>) {

    }
}

impl<C: HasLayouter<LayoutTree = Self>> RenderTree<C> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut tree = Self {
            nodes: HashMap::with_capacity(capacity),
        };

        tree.insert_node(
            NodeId::root(),
            RenderTreeNode {
                id: NodeId::root(),
                layout: <C::Layouter as Layouter>::Layout::default(),
            },
        );

        tree
    }

    /// Inserts a new node into the render tree, note that you are responsible for the node id
    /// and the children of the node
    pub fn insert_node(&mut self, id: NodeId, node: RenderTreeNode<C>) {
        self.nodes.insert(id, node);
    }
}

pub struct RenderTreeNode<C: HasLayouter> {
    pub id: NodeId,
    pub layout: <C::Layouter as Layouter>::Layout,
}

impl<C: HasLayouter> Debug for RenderTreeNode<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderTreeNode")
            .finish()
    }
}

/// Generates a render tree for the given document based on its loaded stylesheets
pub fn generate_render_tree<C: HasDocument + HasRenderTree<LayoutTree = RenderTree<C>, RenderTree = RenderTree<C>>>(
    document: DocumentHandle<C>,
) -> Result<RenderTree<C>> {
    let render_tree = RenderTree::from_document(document);

    Ok(render_tree)
}
