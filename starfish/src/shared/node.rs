
use derive_more::Display;

#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, PartialOrd)]
pub struct NodeId(usize);

impl NodeId {
    pub const ROOT_NODE: usize = 0;
    
    /// Returns the root node ID
    pub fn root() -> Self {
        Self(Self::ROOT_NODE)
    }
}
