
use crate::shared::traits::{config::HasDocument, document::DocumentBuilder};

pub struct DocumentBuilderImpl {}

impl<C: HasDocument> DocumentBuilder<C> for DocumentBuilderImpl {

}
