
use crate::{
    render_backend::Brush as TBrush,
    vello::{Color, VelloBackend},
};

use vello::peniko::Brush as VelloBrush;

#[derive(Clone)]
pub struct Brush(pub(crate) VelloBrush);

impl From<VelloBrush> for Brush {
    fn from(brush: VelloBrush) -> Self {
        Brush(brush)
    }
}

impl TBrush<VelloBackend> for Brush {
    fn color(color: Color) -> Self {
        Brush(VelloBrush::Solid(color.0))
    }
}
