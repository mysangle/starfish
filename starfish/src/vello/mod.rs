use std::{
    fmt::Debug,
    sync::Arc
};

use anyhow::{anyhow, Result};
use vello::{
    peniko::Color as VelloColor,
    AaConfig, Renderer as VelloRenderer, RenderParams, Scene as VelloScene,
};

use crate::render_backend::{RenderBackend, RenderRect, Scene as TScene, WindowHandle};

mod border;
mod brush;
mod color;
mod rect;
mod render;
mod scene;
mod transform;

use border::{Border, BorderRadius, BorderRenderOptions, BorderSide};
use brush::Brush;
use color::Color;
use rect::Rect;
use render::{InstanceAdapter, Renderer, RendererOptions, SurfaceWrapper};
use scene::Scene;
use transform::Transform;

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

impl Default for VelloBackend {
    fn default() -> Self {
        Self::new()
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
    type Rect = Rect;           // vello::kurbo::Rect wrapper
    type Border = Border;
    type BorderSide = BorderSide;
    type BorderRadius = BorderRadius;
    type Transform = Transform; // vello::kurbo::Affine wrapper
    type Color = Color;         // vello::peniko::Color wrapper
    type Brush = Brush;         // vello::peniko::Brush wrapper
    type Scene = Scene;         // vello::Scene wrapper
    type WindowData = VelloWindowData;
    type ActiveWindowData<'a> = VelloActiveWindowData<'a>; // Surface Wrapper

    fn draw_rect(&mut self, data: &mut Self::WindowData, rect: &RenderRect<Self>) {
        data.scene.draw_rect(rect);
    }

    fn apply_scene(&mut self, data: &mut Self::WindowData, scene: &Self::Scene, transform: Option<Self::Transform>) {
        data.scene.apply_scene(scene, transform);
    }

    fn reset(&mut self, data: &mut Self::WindowData) {
        data.scene.reset();
    }

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

    fn resize_window(
            &mut self,
            window_data: &mut Self::WindowData,
            active_window_data: &mut Self::ActiveWindowData<'_>,
            size: crate::render_backend::SizeU32,
    ) -> Result<()> {
        window_data
            .adapter
            .resize_surface(&mut active_window_data.surface, size.width, size.height);

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


