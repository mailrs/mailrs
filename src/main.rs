// Prevent console window in addition to Slint window in Windows release builds when, e.g.,
// starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod cli;
mod config;
mod error;

use clap::Parser;
use miette::IntoDiagnostic;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;

struct Guards {
    _append_guard: Option<()>, // TODO
}

fn setup_logging(log_level: Option<tracing::metadata::Level>) -> Guards {
    let mut env_filter = EnvFilter::from_default_env();

    if let Some(log_level) = log_level {
        let level_filter = tracing::metadata::LevelFilter::from_level(log_level);
        let directive = tracing_subscriber::filter::Directive::from(level_filter);
        env_filter = env_filter.add_directive(directive);
    }

    let subscriber = tracing_subscriber::registry::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_filter(env_filter));

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("Failed to set global logging subscriber: {:?}", e);
        std::process::exit(1)
    }

    Guards {
        _append_guard: None,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), miette::Error> {
    human_panic::setup_panic!(human_panic::Metadata::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
    .authors("Matthias Beyer <mail@beyermatthias.de>"));

    let cli = crate::cli::Cli::parse();
    let _guards = setup_logging(cli.verbosity.tracing_level());
    tracing::debug!(?cli, "Found CLI");

    let config = crate::config::Config::find(cli.config.clone())
        .await
        .map_err(crate::error::ApplicationError::from)
        .into_diagnostic()?;
    tracing::debug!(?config, "Found configuration");

    crate::app::start(cli, config).await.into_diagnostic()?;
    Ok(())
}
