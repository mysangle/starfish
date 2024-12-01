
use crate::{
    render_backend::{
        Border as TBorder, BorderRadius as TBorderRadius, BorderSide as TBorderSide,
        BorderStyle, FP, Radius, RenderBorder,
    },
    vello::{Brush, Rect, VelloBackend, Transform},
};

use vello::{kurbo::RoundedRectRadii, Scene};

pub struct Border {
    pub(crate) left: Option<BorderSide>,
    pub(crate) right: Option<BorderSide>,
    pub(crate) top: Option<BorderSide>,
    pub(crate) bottom: Option<BorderSide>,
}

impl TBorder<VelloBackend> for Border {

}

impl Border {
    pub fn draw(scene: &mut Scene, opts: BorderRenderOptions) {

    }
}

pub struct BorderRenderOptions<'a> {
    pub border: &'a RenderBorder<VelloBackend>,
    pub rect: &'a Rect,
    pub transform: Option<&'a Transform>,
    pub radius: Option<&'a BorderRadius>,
}

#[derive(Clone)]
pub struct BorderSide {
    pub(crate) width: FP,
    pub(crate) style: BorderStyle,
    pub(crate) brush: Brush,
}

impl TBorderSide<VelloBackend> for BorderSide {

}

#[derive(Clone)]
pub struct BorderRadius {
    pub(crate) top_left: Radius,
    pub(crate) top_right: Radius,
    pub(crate) bottom_left: Radius,
    pub(crate) bottom_right: Radius,
}

impl From<[FP; 4]> for BorderRadius {
    fn from(value: [FP; 4]) -> Self {
        Self {
            top_left: value[0].into(),
            top_right: value[1].into(),
            bottom_left: value[2].into(),
            bottom_right: value[3].into(),
        }
    }
}

impl From<[FP; 8]> for BorderRadius {
    fn from(value: [FP; 8]) -> Self {
        Self {
            top_left: (value[0], value[1]).into(),
            top_right: (value[2], value[3]).into(),
            bottom_left: (value[4], value[5]).into(),
            bottom_right: (value[6], value[7]).into(),
        }
    }
}

impl From<(FP, FP, FP, FP)> for BorderRadius {
    fn from(value: (FP, FP, FP, FP)) -> Self {
        Self {
            top_left: value.0.into(),
            top_right: value.1.into(),
            bottom_left: value.2.into(),
            bottom_right: value.3.into(),
        }
    }
}

impl From<(FP, FP, FP, FP, FP, FP, FP, FP)> for BorderRadius {
    fn from(value: (FP, FP, FP, FP, FP, FP, FP, FP)) -> Self {
        Self {
            top_left: (value.0, value.1).into(),
            top_right: (value.2, value.3).into(),
            bottom_left: (value.4, value.5).into(),
            bottom_right: (value.6, value.7).into(),
        }
    }
}

impl From<FP> for BorderRadius {
    fn from(value: FP) -> Self {
        Self {
            top_left: value.into(),
            top_right: value.into(),
            bottom_left: value.into(),
            bottom_right: value.into(),
        }
    }
}

impl From<Radius> for BorderRadius {
    fn from(value: Radius) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_left: value,
            bottom_right: value,
        }
    }
}

impl From<[Radius; 4]> for BorderRadius {
    fn from(value: [Radius; 4]) -> Self {
        Self {
            top_left: value[0],
            top_right: value[1],
            bottom_left: value[2],
            bottom_right: value[3],
        }
    }
}

impl From<(Radius, Radius, Radius, Radius)> for BorderRadius {
    fn from(value: (Radius, Radius, Radius, Radius)) -> Self {
        Self {
            top_left: value.0,
            top_right: value.1,
            bottom_left: value.2,
            bottom_right: value.3,
        }
    }
}

impl TBorderRadius for BorderRadius {

}

impl From<BorderRadius> for RoundedRectRadii {
    fn from(value: BorderRadius) -> Self {
        RoundedRectRadii::new(
            value.top_left.into(),
            value.top_right.into(),
            value.bottom_right.into(),
            value.bottom_left.into(),
        )
    }
}
