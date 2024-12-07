use std::fmt::Debug;

use crate::shared::{
    node::NodeId,
    traits::config::HasDocument,
};

pub trait Node<C: HasDocument>: Clone + Debug + PartialEq {
    fn id(&self) -> NodeId;
}
