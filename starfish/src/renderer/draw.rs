
use crate::{
    interface::{
        config::{HasDrawComponents, HasHtmlParser},
        draw::TreeDrawer,
        render_backend::{
            Brush, Color, Rect, RenderBackend, RenderRect, Scene as TScene, WindowedEventLoop,
        },
    },
    renderer::render_tree::load_html_rendertree,
    shared::{
        geo::{FP, SizeU32},
        types::Result,
    },
    util::render_tree::RenderTree,
};

use url::Url;

#[derive(Debug)]
pub struct TreeDrawerImpl<C: HasDrawComponents> {
    pub(crate) tree: C::RenderTree,
    pub(crate) size: Option<SizeU32>,
    pub(crate) dirty: bool,
    pub(crate) tree_scene: Option<<C::RenderBackend as RenderBackend>::Scene>,
    pub(crate) scene_transform: Option<<C::RenderBackend as RenderBackend>::Transform>,
}

impl<C: HasDrawComponents<RenderTree = RenderTree<C>, LayoutTree = RenderTree<C>> + HasHtmlParser> TreeDrawer<C> for TreeDrawerImpl<C> {
    fn draw(
        &mut self,
        backend: &mut C::RenderBackend,
        data: &mut <C::RenderBackend as RenderBackend>::WindowData,
        size: SizeU32,
        el: &impl WindowedEventLoop,
    ) -> bool {
        if self.tree_scene.is_none() || self.size != Some(size) || !self.dirty {
            self.size = Some(size);

            let mut scene = <C::RenderBackend as RenderBackend>::Scene::new();

            let mut drawer = Drawer {
                scene: &mut scene,
                drawer: self,
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
        let (rt, fetcher) = load_html_rendertree::<C>(url.clone()).await?;

        Ok(Self::new(rt))
    }
}

impl<C: HasDrawComponents> TreeDrawerImpl<C> {
    pub fn new(tree: C::RenderTree) -> Self {
        Self {
            tree,
            size: None,
            dirty: false,
            tree_scene: None,
            scene_transform: None,
        }
    }
}

pub struct Drawer<'s, 't, C: HasDrawComponents> {
    scene: &'s mut <C::RenderBackend as RenderBackend>::Scene,
    drawer: &'t mut TreeDrawerImpl<C>,
}

impl<C: HasDrawComponents> Drawer<'_, '_, C> {
    fn render(&mut self, size: SizeU32) {

    }
}
