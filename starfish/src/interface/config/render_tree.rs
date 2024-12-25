
use crate::interface::{
    config::HasLayouter,
    render_tree::RenderTree,
};

pub trait HasRenderTree: HasLayouter {
    type RenderTree: RenderTree<Self>;
}
