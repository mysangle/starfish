
use crate::{
    render_backend::RenderBackend,
    renderer::draw::SceneDrawer,
    shared::types::Result,
};

use slotmap::{DefaultKey, SlotMap};
use url::Url;

pub struct Tabs<D: SceneDrawer<B>, B: RenderBackend> {
    pub tabs: SlotMap<DefaultKey, Tab<D, B>>,
    pub active: TabID,
}

impl<D: SceneDrawer<B>, B: RenderBackend> Default for Tabs<D, B> {
    fn default() -> Self {
        Self {
            tabs: SlotMap::new(),
            active: TabID::default(),
        }
    }
}

impl<D: SceneDrawer<B>, B: RenderBackend> Tabs<D, B> {
    pub fn add_tab(&mut self, tab: Tab<D, B>) -> TabID {
        TabID(self.tabs.insert(tab))
    }

    pub fn remove_tab(&mut self, id: TabID) {
        self.tabs.remove(id.0);
    }

    pub fn activate_tab(&mut self, id: TabID) {
        self.active = id;
    }

    pub fn get_current_tab(&mut self) -> Option<&mut Tab<D, B>> {
        self.tabs.get_mut(self.active.0)
    }
}

#[derive(Debug)]
pub struct Tab<D: SceneDrawer<B>, B: RenderBackend> {
    pub title: String,
    pub url: Url,
    pub data: D,
    #[allow(clippy::type_complexity)]
    _marker: std::marker::PhantomData<fn(B)>,
}

impl<D: SceneDrawer<B>, B: RenderBackend> Tab<D, B> {
    pub async fn from_url(url: Url) -> Result<Self> {
        let data = D::from_url(url.clone()).await?;

        tracing::info!("Tab created: {}", url.as_str()); 
        Ok(Self {
            title: url.as_str().to_string(),
            url: url,
            data,
            _marker: std::marker::PhantomData,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TabID(pub(crate) DefaultKey);
