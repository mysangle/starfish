
pub use peniko;
mod scene;

pub enum AaConfig {
    Area,
    Msaa8,
    Msaa16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AaSupport {
    pub area: bool,
    pub msaa8: bool,
    pub msaa16: bool,
}

pub struct Renderer {

}

pub struct RenderParams {
    pub base_color: peniko::Color,
    pub width: u32,
    pub height: u32,
    pub antialiasing_method: AaConfig,
}

pub struct RenderOptions {
    
}
