use std::fmt::{Debug, Formatter};

use crate::render_backend::{RenderBackend, Scene as TScene};

use vello::Scene as VelloScene;

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

impl<B: RenderBackend> TScene<B> for Scene {
    fn new() -> Self {
        VelloScene::new().into()
    }
}
