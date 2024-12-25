
use derive_more::Display;

#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, PartialOrd)]
pub struct NodeId(usize);

impl NodeId {
    pub const ROOT_NODE: usize = 0;
    
    /// Returns the root node ID
    pub fn root() -> Self {
        Self(Self::ROOT_NODE)
    }

    /// Returns true when this nodeId is the root node
    pub fn is_root(&self) -> bool {
        self.0 == Self::ROOT_NODE
    }

    // Returns the next node ID
    #[must_use]
    pub fn next(&self) -> Self {
        if self.0 == usize::MAX {
            return Self(usize::MAX);
        }

        Self(self.0 + 1)
    }
}

impl Default for &NodeId {
    /// Returns the default NodeId, which is 0
    fn default() -> Self {
        &NodeId(0)
    }
}
