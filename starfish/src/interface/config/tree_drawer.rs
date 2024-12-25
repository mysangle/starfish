
use crate::interface::{
    config::HasDrawComponents,
    draw::TreeDrawer,
};

pub trait HasTreeDrawer: HasDrawComponents {
    type TreeDrawer: TreeDrawer<Self>;
}
