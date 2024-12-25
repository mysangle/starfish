
use anyhow::anyhow;
use starfish::{
    application::{Application, WindowOptions},
    css3::system::Css3System,
    html5::{
        document::{
            builder::DocumentBuilderImpl,
            document_impl::DocumentImpl,
            fragment::DocumentFragmentImpl,
        },
        parser::Html5Parser,
    },
    interface::config::{
        HasCssSystem, HasDocument, HasHtmlParser, HasLayouter, HasRenderBackend, HasRenderTree, HasTreeDrawer, ModuleConfiguration,
    },
    renderer::draw::TreeDrawerImpl,
    taffy::TaffyLayouter,
    util::render_tree::RenderTree,
    vello::VelloBackend,
};

use clap::ArgAction;
use url::Url;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const LOG_LEVEL_PERMITTED: [&str; 5] = ["trace", "debug", "info", "warn", "error"];

// Application -> [Window] -> Tab -> SceneDrawer
//             -> RenderBackend

#[derive(Clone, Debug, PartialEq)]
struct Config;

impl HasCssSystem for Config {
    type CssSystem = Css3System;
}

impl HasDocument for Config {
    type Document = DocumentImpl<Self>;
    type DocumentFragment = DocumentFragmentImpl<Self>;
    type DocumentBuilder = DocumentBuilderImpl;
}

impl HasHtmlParser for Config {
    type HtmlParser = Html5Parser<'static, Self>;
}

impl HasLayouter for Config {
    type Layouter = TaffyLayouter;
    type LayoutTree = RenderTree<Self>;
}

impl HasRenderTree for Config {
    type RenderTree = RenderTree<Self>;
}

impl HasTreeDrawer for Config {
    type TreeDrawer = TreeDrawerImpl<Self>;
}

impl HasRenderBackend for Config {
    type RenderBackend = VelloBackend;
}

impl ModuleConfiguration for Config {}

fn main() -> anyhow::Result<()> {
    // check command line argument
    let matches = clap::Command::new("Starfish Sleep")
        .arg(
            clap::Arg::new("url")
                .help("The url or file to parse")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::new("log-level")
                .help(format!("Log level: one of {:?}", LOG_LEVEL_PERMITTED))
                .short('l')
                .long("log-level")
                .action(ArgAction::Set),
        )
        .get_matches();

    let url: String = matches.get_one::<String>("url").expect("url").to_string();
    let url = Url::parse(&url)?;
    let log_level = matches.get_one::<String>("log-level").unwrap_or(&"info".to_string()).to_string();
    if !LOG_LEVEL_PERMITTED.iter().any(|item| log_level == *item) {
        return Err(anyhow!(format!("invalid log level: {}", log_level)));
    }

    // check logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}={}", env!("CARGO_CRATE_NAME"), log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Hello, Starfish Sleep!");

    let mut application: Application<Config> = Application::new(VelloBackend::new());
    let opts = WindowOptions::new()
        .with_title("Starfish Sleep")
        .with_size(1024, 768);
    application.initial_tab(url, opts);
    application.run()?;

    Ok(())
}
