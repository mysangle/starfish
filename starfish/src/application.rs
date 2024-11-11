use std::collections::HashMap;

use crate::{
    event::StarfishEvent, render_backend::RenderBackend, renderer::draw::SceneDrawer, shared::types::Result, tabs::Tab, window::Window
};

use anyhow::anyhow;
use url::Url;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    event::{DeviceEvent, DeviceId, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::WindowId,
};

#[derive(Debug, Default)]
pub struct WindowOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl WindowOptions {
    pub fn new() -> Self {
        Self {
            title: "No Title".to_string(),
            width: 1024,
            height: 768,
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> WindowOptions {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }
}

pub struct Application<'a, D: SceneDrawer<B>, B: RenderBackend> {
    windows: HashMap<WindowId, Window<'a, D, B>>,
    open_windows: Vec<(Url, WindowOptions)>,
    backend: B,
    #[allow(clippy::type_complexity)]
    proxy: Option<EventLoopProxy<StarfishEvent<D, B>>>,
}

impl<'a, D: SceneDrawer<B>, B: RenderBackend> Application<'a, D, B> {
    pub fn new(backend: B) -> Self {
        Self {
            windows: HashMap::new(),
            open_windows: Vec::new(),
            backend,
            proxy: None,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let event_loop = EventLoop::with_user_event().build()?;
        self.proxy = Some(event_loop.create_proxy());
        let proxy = self.proxy.clone().ok_or(anyhow!("No proxy; unreachable!"))?;

        tracing::info!("Sending OpenInitial event");
        proxy
            .send_event(StarfishEvent::OpenInitial)
            .map_err(|e| anyhow!(e.to_string()))?;
        event_loop.run_app(self)?;
        
        Ok(())
    }

    pub fn initial_tab(&mut self, url: Url, opts: WindowOptions) {
        self.open_windows.push((url, opts));
    }
}

impl<'a, D: SceneDrawer<B>, B: RenderBackend> ApplicationHandler<StarfishEvent<D, B>> for Application<'a, D, B> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        let _ = (event_loop, cause);
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::info!("Resumed");
        let _ = event_loop;
        for window in self.windows.values_mut() {
            if let Err(e) = window.resumed(&mut self.backend) {
                tracing::error!("Error resuming window: {e:?}");
            }
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: StarfishEvent<D, B>) {
        match event {
            StarfishEvent::OpenInitial => {
                tracing::info!("Opening initial windows");

                for (url, opts) in self.open_windows.drain(..) {
                    // create window
                    let mut window = match Window::new(event_loop, &mut self.backend, opts, self.proxy.clone().unwrap()) {
                        Ok(window) => window,
                        Err(e) => {
                            tracing::error!("Error opening window: {e:?}");
                            if self.windows.is_empty() {
                                tracing::info!("No more windows; exiting event loop");
                                event_loop.exit();;
                            }
                            return;
                        }
                    };

                    if let Err(e) = window.resumed(&mut self.backend) {
                        tracing::error!("Error resuming window: {e:?}");
                        if self.windows.is_empty() {
                            tracing::info!("No more windows; exiting event loop");
                            event_loop.exit();
                        }
                        return;
                    }

                    let id = window.id();
                    self.windows.insert(id, window);

                    let Some(proxy) = self.proxy.clone() else {
                        tracing::error!("No proxy; unreachable!");
                        return;
                    };
                    tracing::info!("Sending OpenTab event");
                    let _ = proxy.send_event(StarfishEvent::OpenTab(url, id));
                }
            },
            StarfishEvent::OpenTab(url, id) => {
                tracing::info!("Opening tab with URL: {url}");
                let Some(proxy) = self.proxy.clone() else {
                    return;
                };

                std::thread::spawn(move || {
                    futures::executor::block_on(async move {
                        let tab = match Tab::from_url(url).await {
                            Ok(tab) => tab,
                            Err(e) => {
                                tracing::error!("Error opening tab: {e:?}");
                                return;
                            }
                        };
                        let _ = proxy.send_event(StarfishEvent::AddTab(tab, id));
                    });
                });
            },
            StarfishEvent::AddTab(tab, id) => {
                tracing::info!("Adding tab to window: {id:?}");

                if let Some(window) = self.windows.get_mut(&id) {
                    tracing::info!("Found window, adding tab");

                    window.add_tab(tab);
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        if let Some(window) = self.windows.get_mut(&window_id) {
            if let Err(e) = window.event(event_loop, &mut self.backend, event) {
                tracing::error!("Error handling window event: {e:?}");
            }
        }
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        let _ = (event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        tracing::info!("Suspended");
        let _ = event_loop;
        for window in self.windows.values_mut() {
            window.suspended(&mut self.backend);
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        tracing::info!("Exiting");
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        tracing::info!("Memory_warning");
        let _ = event_loop;
    }
}
