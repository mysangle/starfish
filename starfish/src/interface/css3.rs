use std::fmt::Debug;

/// Defines the origin of the stylesheet (or declaration)
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CssOrigin {
    /// Browser/user agent defined stylesheets
    UserAgent,
    /// Author defined stylesheets that are linked or embedded in the HTML files
    Author,
    /// User defined stylesheets that will override the author and user agent stylesheets (for instance, custom user styles or extensions)
    User,
}

pub trait CssSystem: Clone + Debug + 'static {
    type Stylesheet: CssStylesheet;

    fn load_default_useragent_stylesheet() -> Self::Stylesheet;
}

pub trait CssStylesheet: PartialEq + Debug {

}
