use std::sync::Arc;

use futures::StreamExt;
use model::MBox;
use model::Message;
use model::Tag;
use notmuch_async::NotmuchWorkerHandle;

mod app;
mod bindings;
mod commands;
mod context;
pub mod error;
mod focus;
mod model;
mod widgets;

pub fn init_logger<S>(level: tracing::Level) -> impl tracing_subscriber::Layer<S>
where
    S: tracing::Subscriber,
    S: for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    if let Err(error) = tui_logger::init_logger({
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

    tui_logger::TuiTracingSubscriberLayer
}

pub async fn run(
    init_query: Option<String>,
    default_query: String,
    notmuch: NotmuchWorkerHandle,
) -> Result<(), self::error::Error> {
    init_panic_hook();
    tracing::debug!("Installed panic hook");

    let terminal = init_tui()?;
    tracing::debug!("Initialized TUI");

    let initial_box = {
        let startup_query = init_query.as_ref().unwrap_or(&default_query);
        tracing::debug!(?startup_query);
        let messages = notmuch
            .create_query(startup_query)
            .search_messages()
            .await
            .map_err(crate::error::Error::from)?
            .into_iter()
            .map(|message| {
                let notmuch = notmuch.clone();

                async move {
                    let tags = notmuch
                        .tags_for_message(&message)
                        .await?
                        .unwrap_or_default();

                    let from = match message.header("From").await {
                        Ok(someornone) => someornone,
                        Err(error) => {
                            tracing::error!(
                                ?error,
                                id = message.id(),
                                "Failed to fetch 'From' header for message"
                            );
                            None
                        }
                    };

                    let subject = match message.header("Subject").await {
                        Ok(someornone) => someornone,
                        Err(error) => {
                            tracing::error!(
                                ?error,
                                id = message.id(),
                                "Failed to fetch 'Subject' header for message"
                            );
                            None
                        }
                    };

                    tracing::info!(id = ?message.id(), ?tags, "Found message");

                    let body = match message.content().await.map(model::Body::new) {
                        Ok(body) => body,
                        Err(error) => {
                            tracing::error!(
                                ?error,
                                id = message.id(),
                                "Failed to fetch 'body' header for message"
                            );
                            model::Body::new(None)
                        }
                    };
                    tracing::info!(id = ?message.id(), "Found body");

                    Ok(Message {
                        id: message.id().to_string(),
                        from,
                        subject,
                        tags: tags
                            .into_iter()
                            .map(|name| Tag {
                                name: name.to_string(),
                            })
                            .collect::<Vec<Tag>>(),
                        body,
                    })
                }
            })
            .collect::<futures::stream::FuturesUnordered<_>>()
            .collect::<Vec<Result<Message, crate::error::Error>>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        MBox::new(startup_query.to_string(), messages)
    };

    let tui_context = self::context::TuiContext::new(notmuch);

    let res = self::app::App::new(Arc::new(initial_box), tui_context)
        .run(terminal)
        .await;

    restore_tui()?;
    res
}

fn init_tui() -> std::io::Result<ratatui::Terminal<impl ratatui::prelude::Backend>> {
    ratatui::crossterm::terminal::enable_raw_mode()?;
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::terminal::EnterAlternateScreen
    )?;
    ratatui::Terminal::new(ratatui::prelude::CrosstermBackend::new(std::io::stdout()))
}

fn init_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

fn restore_tui() -> std::io::Result<()> {
    ratatui::crossterm::terminal::disable_raw_mode()?;
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::terminal::LeaveAlternateScreen
    )?;
    Ok(())
}
