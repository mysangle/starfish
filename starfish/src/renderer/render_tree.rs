
use crate::{
    net::http::fetcher::Fetcher,
    shared::{
        traits::config::{HasHtmlParser, HasRenderTree},
        types::Result,
    },
    util::render_tree::RenderTree,
};

use anyhow::bail;
use url::Url;

pub(crate) async fn load_html_rendertree<
    C: HasRenderTree<LayoutTree = RenderTree<C>, RenderTree = RenderTree<C>> + HasHtmlParser,
>(
    url: Url,
) -> Result<(RenderTree<C>, Fetcher)> {
    let fetcher = Fetcher::new(url.clone());

    let rt = load_html_rendertree_fetcher::<C>(url, &fetcher).await?;

    Ok((rt, fetcher))
}

pub(crate) async fn load_html_rendertree_fetcher<
    C: HasRenderTree<LayoutTree = RenderTree<C>, RenderTree = RenderTree<C>> + HasHtmlParser,
>(
    url: Url,
    fetcher: &Fetcher,
) -> Result<RenderTree<C>> {
    // Fetch the html from the url
    let response = fetcher.get(url.as_ref()).await?;
    if response.status != 200 {
        bail!(format!("Could not get url. Status code {}", response.status));
    }

    let html = String::from_utf8(response.body.clone())?;
    tracing::info!("\n{}", html);

    Ok(RenderTree::with_capacity(1))
}

