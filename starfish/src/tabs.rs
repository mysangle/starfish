
use crate::shared::{
    traits::{config::ModuleConfiguration, draw::TreeDrawer},
    types::Result,
};

use slotmap::{DefaultKey, SlotMap};
use url::Url;

pub struct Tabs<C: ModuleConfiguration> {
    #[allow(clippy::type_complexity)]
    pub tabs: SlotMap<DefaultKey, Tab<C>>,
    pub active: TabID,
}

impl<C: ModuleConfiguration> Default for Tabs<C> {
    fn default() -> Self {
        Self {
            tabs: SlotMap::new(),
            active: TabID::default(),
        }
    }
}

impl<C: ModuleConfiguration> Tabs<C> {
    pub fn add_tab(&mut self, tab: Tab<C>) -> TabID {
        TabID(self.tabs.insert(tab))
    }

    pub fn remove_tab(&mut self, id: TabID) {
        self.tabs.remove(id.0);
    }

    pub fn activate_tab(&mut self, id: TabID) {
        self.active = id;
    }

    pub fn get_current_tab(&mut self) -> Option<&mut Tab<C>> {
        self.tabs.get_mut(self.active.0)
    }
}

#[derive(Debug)]
pub struct Tab<C: ModuleConfiguration> {
    pub title: String,
    pub url: Url,
    pub data: C::TreeDrawer,
}

impl<C: ModuleConfiguration> Tab<C> {
    pub async fn from_url(url: Url) -> Result<Self> {
        let data = C::TreeDrawer::from_url(url.clone()).await?;

        tracing::info!("Tab created: {}", url.as_str()); 
        Ok(Self {
            title: url.as_str().to_string(),
            url,
            data,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TabID(pub(crate) DefaultKey);
