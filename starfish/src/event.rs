use std::fmt::{Debug, Formatter};

use crate::{
    shared::traits::config::ModuleConfiguration,
    tabs::Tab,
};

use url::Url;
use winit::window::WindowId;

pub enum StarfishEvent<C: ModuleConfiguration> {
    // Window 생성
    OpenInitial,
    // Tab 생성
    OpenTab(Url, WindowId),
    // Tab을 Window에 추가
    AddTab(Tab<C>, WindowId),
}

impl<C: ModuleConfiguration> Debug for StarfishEvent<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenInitial => f.write_str("OpenInitial"),
            Self::OpenTab(..) => f.write_str("OpenTab"),
            Self::AddTab(..) => f.write_str("AddTab"),
        }
    }
}
