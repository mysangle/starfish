use std::fmt::{self, Debug, Formatter};

#[derive(PartialEq, Clone)]
pub struct DocTypeData {
    pub name: String,
    pub pub_identifier: String,
    pub sys_identifier: String,
}

impl Default for DocTypeData {
    fn default() -> Self {
        Self::new("", "", "")
    }
}

impl Debug for DocTypeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("DocTypeData");
        debug.finish()
    }
}

impl DocTypeData {
    #[must_use]
    pub fn new(name: &str, pub_identifier: &str, sys_identifier: &str) -> Self {
        Self {
            name: name.to_owned(),
            pub_identifier: pub_identifier.to_owned(),
            sys_identifier: sys_identifier.to_owned(),
        }
    }
}
