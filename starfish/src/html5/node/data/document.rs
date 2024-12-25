use std::fmt::{self, Debug, Formatter};

use crate::interface::node::QuirksMode;

#[derive(PartialEq, Clone)]
/// Data structure for document nodes
pub struct DocumentData {
    quirks_mode: QuirksMode,
}

impl Debug for DocumentData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("DocumentData");
        debug.finish()
    }
}

impl DocumentData {
    #[must_use]
    pub(crate) fn new(quirks_mode: QuirksMode) -> Self {
        Self { quirks_mode }
    }
}
