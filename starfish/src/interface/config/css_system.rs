
use crate::interface::css3::{CssSystem, CssStylesheet};

pub trait HasCssSystem:
    Sized
    + HasCssSystemExt<
        Self,
        Stylesheet = <Self::CssSystem as CssSystem>::Stylesheet,
    >
{
    type CssSystem: CssSystem;
}

pub trait HasCssSystemExt<C: HasCssSystem> {
    type Stylesheet: CssStylesheet;
}

impl<C: HasCssSystem> HasCssSystemExt<C> for C {
    type Stylesheet = <C::CssSystem as CssSystem>::Stylesheet;
}
