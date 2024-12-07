
use crate::shared::traits::config::HasLayouter;
use crate::shared::traits::render_tree::RenderTree;

pub trait HasRenderTree: HasLayouter {
    type RenderTree: RenderTree<Self>;
}
