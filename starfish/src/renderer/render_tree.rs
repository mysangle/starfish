
use crate::{
    render_backend::{RenderBackend, Scene as TScene, SizeU32, WindowedEventLoop},
    renderer::draw::SceneDrawer,
    shared::types::Result,
};

use url::Url;

#[derive(Debug)]
pub struct TreeDrawer<B: RenderBackend> {
    size: Option<SizeU32>,
    dirty: bool,
    tree_scene: Option<B::Scene>,
}

impl<B: RenderBackend> SceneDrawer<B> for TreeDrawer<B> {
    fn draw(
        &mut self,
        backend: &mut B,
        data: &mut B::WindowData,
        size: SizeU32,
        el: &impl WindowedEventLoop,
    ) -> bool {
        if self.tree_scene.is_none() || self.size != Some(size) || !self.dirty {
            self.size = Some(size);

            let mut scene = B::Scene::new();

            let mut drawer = Drawer {
                scene: &mut scene,
                sceneDrawer: self,
            };
            drawer.render(size);

            self.tree_scene = Some(scene);
        }

        false
    }

    async fn from_url(url: Url) -> Result<Self> {
        Ok(Self::new())
    }
}

impl<B: RenderBackend> TreeDrawer<B> {
    pub fn new() -> Self {
        Self {
            size: None,
            dirty: false,
            tree_scene: None,
        }
    }
}

pub struct Drawer<'s, B: RenderBackend> {
    scene: &'s mut B::Scene,
    sceneDrawer: &'s mut TreeDrawer<B>,
}

impl<B: RenderBackend> Drawer<'_, B> {
    fn render(&mut self, size: SizeU32) {

    }
}
