// Prevent console window in addition to Slint window in Windows release builds when, e.g.,
// starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(all(not(feature = "gui"), not(feature = "tui")))]
compile_error!("Either 'gui' or 'tui' feature must be enabled!");

mod app;
mod cli;
mod config;
mod error;
mod notmuch;

#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "tui")]
mod tui;

use clap::Parser;
use miette::IntoDiagnostic;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;

#[cfg(feature = "gui")]
slint::include_modules!();

struct Guards {
    _append_guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}

fn setup_logging(cli: &crate::cli::Cli) -> Guards {
    let mut file_filter = EnvFilter::from_default_env();
    let mut tui_filter = EnvFilter::from_default_env();

    if let Some(log_level) = cli.verbosity.tracing_level() {
        let level_filter = tracing::metadata::LevelFilter::from_level(log_level);
        let directive = tracing_subscriber::filter::Directive::from(level_filter);
        tui_filter = tui_filter.add_directive(directive.clone());
        file_filter = file_filter.add_directive(directive);
    }

    let (file_layer, guard) = if let Some(path) = cli.logfile.as_ref() {
        let Some(dir) = path.parent() else {
            eprintln!("Path has no parent: {path}, exiting");
            std::process::exit(1);
        };
        let Some(filename) = path.file_name() else {
            eprintln!("Path has no file name: {path}, exiting");
            std::process::exit(1);
        };

        let file_appender = tracing_appender::rolling::never(dir, filename);

        let (logger, guard) = tracing_appender::non_blocking(file_appender);

        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(logger)
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_ansi(false)
            .with_filter(file_filter);

        (Some(file_layer), Some(guard))
    } else {
        (None, None)
    };

    if let Err(error) = tui_logger::init_logger({
        let level = cli
            .verbosity
            .tracing_level()
            .unwrap_or(tracing::Level::INFO);

        match level {
            tracing::Level::TRACE => log::LevelFilter::Trace,
            tracing::Level::DEBUG => log::LevelFilter::Debug,
            tracing::Level::INFO => log::LevelFilter::Info,
            tracing::Level::WARN => log::LevelFilter::Warn,
            tracing::Level::ERROR => log::LevelFilter::Error,
        }
    }) {
        eprintln!("Failed to initialize TUI logger: {error:?}");
        std::process::exit(1);
    }

    let tui_layer = tui_logger::TuiTracingSubscriberLayer.with_filter(tui_filter);

    if let Err(error) = tracing_subscriber::registry()
        .with(file_layer)
        .with(tui_layer)
        .try_init()
    {
        eprintln!("Failed to initialize logging: {error:?}");
        std::process::exit(1)
    }

    Guards {
        _append_guard: guard,
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
    let _guards = setup_logging(&cli);
    tracing::debug!(?cli, "Found CLI");

    let config = crate::config::Config::find(cli.config.clone())
        .await
        .map_err(crate::error::ApplicationError::from)
        .into_diagnostic()?;
    tracing::debug!(?config, "Found configuration");

    crate::app::start(cli, config).await.into_diagnostic()?;
    Ok(())
}
