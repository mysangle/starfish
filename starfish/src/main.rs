
use anyhow::anyhow;
use starfish::{application::{Application, WindowOptions}, renderer::render_tree::TreeDrawer, vello::VelloBackend};

use clap::ArgAction;
use url::Url;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const LOG_LEVEL_PERMITTED: [&str; 5] = ["trace", "debug", "info", "warn", "error"];

// Application -> [Window] -> Tab -> SceneDrawer
//             -> RenderBackend
type Backend = VelloBackend;
type Drawer = TreeDrawer<Backend>;

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

    let mut application: Application<Drawer, Backend> = Application::new(VelloBackend::new());
    let opts = WindowOptions::new()
        .set_size(1024, 768);
    application.initial_tab(url, opts);
    application.run()?;

    Ok(())
}
