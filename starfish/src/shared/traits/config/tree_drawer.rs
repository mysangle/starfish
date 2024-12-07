
use crate::shared::traits::config::HasDrawComponents;
use crate::shared::traits::draw::TreeDrawer;

pub trait HasTreeDrawer: HasDrawComponents {
    type TreeDrawer: TreeDrawer<Self>;
}
