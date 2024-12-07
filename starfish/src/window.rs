use std::sync::Arc;

use crate::{
    application::WindowOptions,
    event::StarfishEvent,
    shared::{
        render_backend::{RenderBackend, SizeU32, WindowedEventLoop},
        traits::{config::ModuleConfiguration, draw::TreeDrawer},
        types::Result,
    },
    tabs::{Tab, TabID, Tabs},
};

use anyhow::anyhow;
use winit::{
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoopProxy},
    window::{Window as WInitWindow, WindowId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowState<'a, B: RenderBackend> {
    Active { surface: B::ActiveWindowData<'a> },
    Suspended,
}

pub struct Window<'a, C: ModuleConfiguration> {
    state: WindowState<'a, C::RenderBackend>,
    window: Arc<WInitWindow>,
    renderer_data: <C::RenderBackend as RenderBackend>::WindowData,
    tabs: Tabs<C>,
    el: WindowEventLoop<C>,
}

impl<'a, C: ModuleConfiguration> Window<'a, C> {
    pub fn new(event_loop: &ActiveEventLoop, backend: &mut C::RenderBackend, opts: WindowOptions, el: EventLoopProxy<StarfishEvent<C>>) -> Result<Self> {
        let attributes = WInitWindow::default_attributes()
            .with_title(opts.title)
            .with_inner_size(LogicalSize::new(opts.width, opts.height));
        let window = event_loop
            .create_window(attributes)
            .map_err(|e| anyhow!(e.to_string()))?;
        let window = Arc::from(window);

        let renderer_data = backend.create_window_data(window.clone())?;

        let el = WindowEventLoop {
            proxy: el,
            id: window.id(),
        };

        Ok(Self {
            state: WindowState::Suspended,
            window,
            renderer_data,
            tabs: Tabs::default(),
            el,
        })
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn add_tab(&mut self, tab: Tab<C>) {
        let id = self.tabs.add_tab(tab);

        if self.tabs.active == TabID::default() {
            self.tabs.activate_tab(id);
        }

        self.window.request_redraw();
    }

    pub fn resumed(&mut self, backend: &mut C::RenderBackend) -> Result<()> {
        if !matches!(self.state, WindowState::Suspended) {
            return Ok(());
        };

        let size = self.window.inner_size();
        let size = SizeU32::new(size.width, size.height);

        let data = backend.activate_window(self.window.clone(), &mut self.renderer_data, size)?;

        self.state = WindowState::Active { surface: data };

        Ok(())
    }

    pub fn suspended(&mut self, backend: &mut C::RenderBackend) {
        let WindowState::Active { surface: data } = &mut self.state else {
            return;
        };

        if let Err(e) = backend.suspend_window(self.window.clone(), data, &mut self.renderer_data) {
            tracing::warn!("Failed to suspend window: {}", e);
        }

        self.state = WindowState::Suspended;
    }

    pub fn event(&mut self, event_loop: &ActiveEventLoop, backend: &mut C::RenderBackend, event: WindowEvent) -> Result<()> {
        let WindowState::Active { surface: active_window_data } = &mut self.state
        else {
            return Ok(());
        };

        let window = &self.window;

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                backend.resize_window(&mut self.renderer_data, active_window_data, SizeU32::new(size.width, size.height))?;
                window.request_redraw();
            },
            WindowEvent::RedrawRequested => {
                let size = window.inner_size();

                let size = SizeU32::new(size.width, size.height);

                let Some(tab) = self.tabs.get_current_tab() else {
                    return Ok(());
                };

                let redraw = tab.data.draw(backend, &mut self.renderer_data, size, &self.el);

                backend.render(&mut self.renderer_data, active_window_data)?;

                if redraw {
                    self.request_redraw();
                }
            },
            _ => {},
        }
        Ok(())
    }
}

pub(crate) struct WindowEventLoop<C: ModuleConfiguration> {
    proxy: EventLoopProxy<StarfishEvent<C>>,
    id: WindowId,
}

impl<C: ModuleConfiguration> Clone for WindowEventLoop<C> {
    fn clone(&self) -> Self {
        Self {
            proxy: self.proxy.clone(),
            id: self.id,
        }
    }
}

impl<C: ModuleConfiguration> WindowedEventLoop for WindowEventLoop<C> {

}
