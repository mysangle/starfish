use std::{
    fmt::Debug,
    ops::{Mul, MulAssign},
};

use crate::shared::geo::*;

use anyhow::Result;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub trait WindowHandle: HasDisplayHandle + HasWindowHandle + Send + Sync + Clone {}

impl<T> WindowHandle for T where T: HasDisplayHandle + HasWindowHandle + Send + Sync + Clone {}

pub trait WindowedEventLoop: Send + Clone + 'static {

}

pub trait RenderBackend: Sized + Debug + 'static {
    type Rect: Rect;
    type Border: Border<Self>;
    type BorderSide: BorderSide<Self>;
    type BorderRadius: BorderRadius;
    type Transform: Transform;
    type Color: Color;
    type Brush: Brush<Self>;
    type Scene: Scene<Self> + Send;

    type WindowData;
    type ActiveWindowData<'a>;

    fn draw_rect(&mut self, data: &mut Self::WindowData, rect: &RenderRect<Self>);
    fn apply_scene(&mut self, data: &mut Self::WindowData, scene: &Self::Scene, transform: Option<Self::Transform>);
    fn reset(&mut self, data: &mut Self::WindowData);

    fn create_window_data(&mut self, handle: impl WindowHandle) -> Result<Self::WindowData>;

    fn activate_window<'a>(
        &mut self,
        handle: impl WindowHandle + 'a,
        data: &mut Self::WindowData,
        size: SizeU32,
    ) -> Result<Self::ActiveWindowData<'a>>;

    fn suspend_window(
        &mut self,
        handle: impl WindowHandle,
        data: &mut Self::ActiveWindowData<'_>,
        window_data: &mut Self::WindowData,
    ) -> Result<()>;

    fn resize_window(
        &mut self,
        window_data: &mut Self::WindowData,
        active_window_data: &mut Self::ActiveWindowData<'_>,
        size: SizeU32,
    ) -> Result<()>;

    fn render(
        &mut self,
        window_data: &mut Self::WindowData,
        active_data: &mut Self::ActiveWindowData<'_>,
    ) -> Result<()>;
}

pub trait Scene<B: RenderBackend>: Clone + Debug {
    fn draw_rect(&mut self, rect: &RenderRect<B>);
    fn apply_scene(&mut self, scene: &B::Scene, transform: Option<B::Transform>);
    fn reset(&mut self);
    fn new() -> Self;
}

pub trait Rect {
    fn new(x: FP, y: FP, width: FP, height: FP) -> Self;
}

pub trait Border<B: RenderBackend> {

}

pub trait BorderSide<B: RenderBackend> {

}

#[derive(Clone, Copy)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    None,
    Hidden,
}

pub trait BorderRadius:
    Sized
    + From<FP>
    + From<Radius>
    + From<[FP; 4]>
    + From<[Radius; 4]>
    + From<[FP; 8]>
    + From<(FP, FP, FP, FP)>
    + From<(Radius, Radius, Radius, Radius)>
    + From<(FP, FP, FP, FP, FP, FP, FP, FP)>
{

}

impl From<FP> for Radius {
    fn from(value: FP) -> Self {
        Radius::Uniform(value)
    }
}

impl From<[FP; 2]> for Radius {
    fn from(value: [FP; 2]) -> Self {
        Radius::Elliptical(value[0], value[1])
    }
}

impl From<(FP, FP)> for Radius {
    fn from(value: (FP, FP)) -> Self {
        Radius::Elliptical(value.0, value.1)
    }
}

impl From<Radius> for (f64, f64) {
    fn from(value: Radius) -> Self {
        match value {
            Radius::Uniform(value) => (value as f64, value as f64),
            Radius::Elliptical(x, y) => (x as f64, y as f64),
        }
    }
}

impl From<Radius> for f64 {
    fn from(value: Radius) -> Self {
        match value {
            Radius::Uniform(value) => value as f64,
            Radius::Elliptical(x, y) => (x * y).sqrt() as f64,
        }
    }
}

impl From<Radius> for FP {
    fn from(value: Radius) -> Self {
        match value {
            Radius::Uniform(value) => value,
            Radius::Elliptical(x, y) => (x * y).sqrt(),
        }
    }
}

impl From<Radius> for [FP; 2] {
    fn from(value: Radius) -> Self {
        match value {
            Radius::Uniform(value) => [value, value],
            Radius::Elliptical(x, y) => [x, y],
        }
    }
}

impl From<Radius> for (FP, FP) {
    fn from(value: Radius) -> Self {
        match value {
            Radius::Uniform(value) => (value, value),
            Radius::Elliptical(x, y) => (x, y),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Radius {
    Uniform(FP),
    Elliptical(FP, FP),
}

pub struct RenderRect<B: RenderBackend> {
    pub rect: B::Rect,
    pub transform: Option<B::Transform>,
    pub radius: Option<B::BorderRadius>,
    pub brush: B::Brush,
    pub brush_transform: Option<B::Transform>,
    pub border: Option<RenderBorder<B>>,
}

pub struct RenderBorder<B: RenderBackend> {
    pub border: B::Border,
    pub transform: Option<B::Transform>,
}

pub trait Transform: Sized + Mul<Self> + MulAssign + Clone + Send + Debug {

}

pub trait Brush<B: RenderBackend>: Clone {
    fn color(color: B::Color) -> Self;
}

pub trait Color {
    fn new(r: u8, g: u8, b: u8) -> Self
    where
        Self: Sized,
    {
        Self::with_alpha(r, g, b, 255)
    }

    fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self;

    fn rgb(r: u8, g: u8, b: u8) -> Self
    where
        Self: Sized,
    {
        Self::new(r, g, b)
    }

    fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self
    where
        Self: Sized,
    {
        Self::with_alpha(r, g, b, a)
    }

    fn tuple3(tup: (u8, u8, u8)) -> Self
    where
        Self: Sized,
    {
        Self::new(tup.0, tup.1, tup.2)
    }

    fn tuple4(tup: (u8, u8, u8, u8)) -> Self
    where
        Self: Sized,
    {
        Self::with_alpha(tup.0, tup.1, tup.2, tup.3)
    }

    fn alpha(self, a: u8) -> Self
    where
        Self: Sized,
    {
        Self::with_alpha(self.r(), self.g(), self.b(), a)
    }

    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;

    const WHITE: Self;
    const BLACK: Self;
    const RED: Self;
    const GREEN: Self;
    const BLUE: Self;
    const YELLOW: Self;
    const CYAN: Self;
    const MAGENTA: Self;
    const TRANSPARENT: Self;
}
