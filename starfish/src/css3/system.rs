
use crate::{
    css3::{
        stylesheet::CssStylesheet,
        load_default_useragent_stylesheet,
    },
    interface::css3::CssSystem,
};

#[derive(Debug, Clone)]
pub struct Css3System;

impl CssSystem for Css3System {
    type Stylesheet = CssStylesheet;

    fn load_default_useragent_stylesheet() -> Self::Stylesheet {
        load_default_useragent_stylesheet()
    }
}
