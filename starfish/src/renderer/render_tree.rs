
use crate::{
    interface::{
        config::{HasHtmlParser, HasRenderTree},
        document::{Document, DocumentBuilder},
        html5::Html5Parser,
    },
    net::http::fetcher::Fetcher,
    shared::{
        byte_stream::{ByteStream, Encoding},
        document::DocumentHandle,
        types::Result,
    },
    util::render_tree::{generate_render_tree, RenderTree},
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

    let mut stream = ByteStream::new(Encoding::UTF8, None);
    stream.read_from_str(&html, Some(Encoding::UTF8));
    stream.close();

    let mut doc_handle = C::DocumentBuilder::new_document(Some(url));
    let parse_errors = C::HtmlParser::parse(&mut stream, DocumentHandle::clone(&doc_handle), None)?;

    for error in parse_errors {
        eprintln!("Parse error: {:?}", error);
    }

    let mut doc = doc_handle.get_mut();
    //doc.add_stylesheet(C::CssSystem::load_default_useragent_stylesheet());

    drop(doc);

    generate_render_tree(DocumentHandle::clone(&doc_handle))
}

