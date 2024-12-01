
use crate::{
    render_backend::{
        geo::FP,
        Brush, Color, Rect, RenderBackend, RenderRect, Scene as TScene, SizeU32, WindowedEventLoop,
    },
    renderer::draw::SceneDrawer,
    shared::types::Result,
};

use url::Url;

#[derive(Debug)]
pub struct TreeDrawer<B: RenderBackend> {
    size: Option<SizeU32>,
    dirty: bool,
    tree_scene: Option<B::Scene>,
    scene_transform: Option<B::Transform>,
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

        backend.reset(data);

        let bg = Rect::new(0.0, 0.0, size.width as FP, size.height as FP);
        let rect = RenderRect {
            rect: bg,
            transform: None,
            radius: None,
            brush: Brush::color(Color::WHITE),
            brush_transform: None,
            border: None,
        };

        backend.draw_rect(data, &rect);

        if let Some(scene) = &self.tree_scene {
            backend.apply_scene(data, scene, self.scene_transform.clone());
        }

        if self.dirty {
            self.dirty = false;

            return true;
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
            scene_transform: None,
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
