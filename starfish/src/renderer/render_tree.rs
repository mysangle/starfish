use std::future::Future;

use crate::{
    render_backend::{RenderBackend, SizeU32, WindowedEventLoop},
    renderer::draw::SceneDrawer,
    shared::types::Result,
};

use url::Url;

#[derive(Debug)]
pub struct TreeDrawer<B: RenderBackend> {
    pub(crate) tree_scene: Option<B::Scene>,
}

impl<B: RenderBackend> SceneDrawer<B> for TreeDrawer<B> {
    fn draw(
        &mut self,
        backend: &mut B,
        data: &mut B::WindowData,
        size: SizeU32,
        el: &impl WindowedEventLoop,
    ) -> bool {
        false
    }

    async fn from_url(url: Url) -> Result<Self> {
        Ok(Self::new())
    }
}

impl<B: RenderBackend> TreeDrawer<B> {
    pub fn new() -> Self {
        Self {
            tree_scene: None,
        }
    }
}
