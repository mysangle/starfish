use std::fmt::{Debug, Formatter};

use crate::{
    interface::render_backend::{RenderRect, Scene as TScene},
    vello::{Border, BorderRenderOptions, Transform, VelloBackend},
};

use vello::{
    kurbo::RoundedRect,
    peniko::Fill,
    Scene as VelloScene,
};

#[derive(Clone)]
pub struct Scene(pub(crate) VelloScene);

impl Debug for Scene {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scene").finish()
    }
}

impl From<VelloScene> for Scene {
    fn from(scene: VelloScene) -> Self {
        Self(scene)
    }
}

impl TScene<VelloBackend> for Scene {
    fn draw_rect(&mut self, rect: &RenderRect<VelloBackend>) {
        let affine = rect.transform.as_ref().map(|t| t.0).unwrap_or_default();

        let brush = &rect.brush.0;
        let brush_transform = rect.brush_transform.as_ref().map(|t| t.0);

        if let Some(radius) = &rect.radius {
            let shape = RoundedRect::from_rect(rect.rect.0, radius.clone());
            self.0.fill(Fill::NonZero, affine, brush, brush_transform, &shape)
        } else {
            self.0.fill(Fill::NonZero, affine, brush, brush_transform, &rect.rect.0)
        }

        if let Some(border) = &rect.border {
            let opts = BorderRenderOptions {
                border,
                rect: &rect.rect,
                transform: rect.transform.as_ref(),
                radius: rect.radius.as_ref(),
            };

            Border::draw(&mut self.0, opts);
        }
    }

    fn apply_scene(&mut self, scene: &Scene, transform: Option<Transform>) {
        self.0.append(&scene.0, transform.map(|t| t.0));
    }

    fn reset(&mut self) {
        self.0.reset()
    }

    fn new() -> Self {
        VelloScene::new().into()
    }
}
