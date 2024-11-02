use std::fmt::Debug;

use anyhow::Result;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub mod geo;
pub use geo::*;

pub trait WindowHandle: HasDisplayHandle + HasWindowHandle + Send + Sync + Clone {}

impl<T> WindowHandle for T where T: HasDisplayHandle + HasWindowHandle + Send + Sync + Clone {}

pub trait WindowedEventLoop: Send + Clone + 'static {

}

pub trait RenderBackend: Sized + Debug + 'static {
    type Scene: Scene<Self> + Send;
    type WindowData;
    type ActiveWindowData<'a>;

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

    fn render(
        &mut self,
        window_data: &mut Self::WindowData,
        active_data: &mut Self::ActiveWindowData<'_>,
    ) -> Result<()>;
}

pub trait Scene<B: RenderBackend>: Clone + Debug {

}
