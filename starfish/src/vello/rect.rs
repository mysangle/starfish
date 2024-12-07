
use crate::shared::render_backend::{FP, Rect as Trect};

use vello::kurbo::Rect as VelloRect;

pub struct Rect(pub(crate) VelloRect);

impl From<VelloRect> for Rect {
    fn from(rect: VelloRect) -> Self {
        Rect(rect)
    }
}

impl Trect for Rect {
    fn new(x: FP, y: FP, width: FP, height: FP) -> Self {
        VelloRect::new(x as f64, y as f64, x as f64 + width as f64, y as f64 + height as f64).into()
    }
}
