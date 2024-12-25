
use crate::interface::render_backend::RenderBackend;

pub trait HasRenderBackend {
    type RenderBackend: RenderBackend;
}
