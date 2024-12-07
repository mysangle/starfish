use std::future::Future;

use crate::shared::{
    render_backend::{RenderBackend, SizeU32, WindowedEventLoop},
    traits::config::HasDrawComponents,
    types::Result,
};

use url::Url;

pub trait TreeDrawer<C: HasDrawComponents>: Send + 'static {
    fn draw(
        &mut self,
        backend: &mut C::RenderBackend,
        data: &mut <C::RenderBackend as RenderBackend>::WindowData,
        size: SizeU32,
        el: &impl WindowedEventLoop,
    ) -> bool;

    fn from_url(url: Url) -> impl Future<Output = Result<Self>> + Send
    where
        Self: Sized;
}
