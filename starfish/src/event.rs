
use crate::{
    render_backend::RenderBackend,
    renderer::draw::SceneDrawer,
    tabs::Tab,
};

use url::Url;
use winit::window::WindowId;

#[derive(Debug)]
pub enum StarfishEvent<D: SceneDrawer<B>, B: RenderBackend> {
    // Window 생성
    OpenInitial,
    // Tab 생성
    OpenTab(Url, WindowId),
    // Tab을 Window에 추가
    AddTab(Tab<D, B>, WindowId),
}
