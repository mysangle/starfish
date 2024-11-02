use std::{
    fmt::Debug,
    sync::Arc
};

use anyhow::{anyhow, Result};
use vello::{
    peniko::Color as VelloColor,
    {AaConfig, Renderer as VelloRenderer, RenderParams, Scene as VelloScene},
};

use crate::render_backend::{RenderBackend, WindowHandle};

mod render;
mod scene;

use render::{InstanceAdapter, Renderer, RendererOptions, SurfaceWrapper};
use scene::Scene;

pub struct VelloWindowData {
    pub(crate) adapter: Arc<InstanceAdapter>,
    pub(crate) renderer: VelloRenderer,
    pub(crate) scene: Scene,
}

pub struct VelloActiveWindowData<'a> {
    surface: SurfaceWrapper<'a>,
}

pub struct VelloBackend {

}

impl Debug for VelloBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VelloRenderer").finish()
    }
}

impl VelloBackend {
    pub fn new() -> Self {
        Self {}
    }
}

/**
 * create_window_data -> InstanceAdapter
 * activate_window    -> SurfaceWrapper
 * render             -> SurfaceTexture
 */
impl RenderBackend for VelloBackend {
    type Scene = Scene;
    type WindowData = VelloWindowData;
    type ActiveWindowData<'a> = VelloActiveWindowData<'a>;

    fn create_window_data(&mut self, _handle: impl WindowHandle) -> Result<Self::WindowData> {
        tracing::info!("Creating window data");

        let renderer = futures::executor::block_on(Renderer::new(RendererOptions::default()))?;

        let adapter = renderer.instance_adapter;

        let renderer = adapter.create_renderer(None)?;

        tracing::info!("Created renderer");
        Ok(VelloWindowData {
            adapter,
            renderer,
            scene: VelloScene::new().into(),
        })
    }

    fn activate_window<'a>(
            &mut self,
            handle: impl WindowHandle + 'a,
            data: &mut Self::WindowData,
            size: crate::render_backend::SizeU32,
        ) -> Result<Self::ActiveWindowData<'a>> {
        let surface = data
            .adapter
            .create_surface(handle, size.width, size.height, wgpu::PresentMode::AutoVsync)?;

        // 새로 생성한 SurfaceConfiguration에 연결된 VelloRenderer 생성
        let renderer = data.adapter.create_renderer(Some(surface.config.format))?;
        data.renderer = renderer;

        Ok(VelloActiveWindowData { surface })
    }

    fn suspend_window(
            &mut self,
            _handle: impl WindowHandle,
            _data: &mut Self::ActiveWindowData<'_>,
            _window_data: &mut Self::WindowData,
        ) -> Result<()> {
        Ok(())
    }

    fn render(
        &mut self,
        window_data: &mut Self::WindowData,
        active_data: &mut Self::ActiveWindowData<'_>,
    ) -> Result<()> {
        let height = active_data.surface.config.height;
        let width = active_data.surface.config.width;

        let surface_texture = active_data.surface.surface.get_current_texture()?;

        window_data
            .renderer
            .render_to_surface(
                &window_data.adapter.device,
                &window_data.adapter.queue,
                &window_data.scene.0,
                &surface_texture,
                &RenderParams {
                    base_color: VelloColor::WHITE,
                    width,
                    height,
                    antialiasing_method: AaConfig::Msaa16,
                },
            )
            .map_err(|e| anyhow!(e.to_string()))?;

        surface_texture.present();

        Ok(())
    }
}


