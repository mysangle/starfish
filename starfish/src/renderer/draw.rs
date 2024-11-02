use std::future::Future;

use crate::{
    render_backend::{RenderBackend, SizeU32, WindowedEventLoop},
    shared::types::Result,
};

use url::Url;

pub trait SceneDrawer<B: RenderBackend>: Send + 'static {
    fn draw(
        &mut self,
        backend: &mut B,
        data: &mut B::WindowData,
        size: SizeU32,
        el: &impl WindowedEventLoop,
    ) -> bool;

    fn from_url(url: Url) -> impl Future<Output = Result<Self>> + Send
    where
        Self: Sized;
}
