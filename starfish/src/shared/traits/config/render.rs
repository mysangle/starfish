
use crate::shared::render_backend::RenderBackend;

pub trait HasRenderBackend {
    type RenderBackend: RenderBackend;
}
