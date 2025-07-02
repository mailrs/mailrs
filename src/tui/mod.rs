use std::sync::Arc;

use error::AppError;
use futures::StreamExt;
use model::MBox;
use model::Message;
use model::Tag;

use crate::cli::Cli;
use crate::config::Config;
use crate::notmuch::NotmuchWorkerHandle;

mod app;
mod bindings;
mod commands;
mod context;
pub mod error;
mod jobserver;
mod model;
mod widgets;

pub async fn run(
    cli: Cli,
    config: Config,
    notmuch: NotmuchWorkerHandle,
) -> Result<(), self::error::Error> {
    init_panic_hook();
    tracing::debug!("Installed panic hook");

    let terminal = init_tui()?;
    tracing::debug!("Initialized TUI");

    let initial_box = {
        let startup_query = cli.init_query.as_ref().unwrap_or(&config.default_query);
        tracing::debug!(?startup_query);
        let messages = notmuch
            .create_query(startup_query)
            .search_messages()
            .await
            .map_err(AppError::from)?
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
                    })
                }
            })
            .collect::<futures::stream::FuturesUnordered<_>>()
            .collect::<Vec<Result<Message, crate::notmuch::WorkerError<_>>>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map_err(AppError::from)?;

        MBox::new(startup_query.to_string(), messages)
    };

    let tui_context = self::context::TuiContext::new(cli, config, notmuch);

    let mut app = self::app::App::new(tui_context);
    app.add_box(Arc::new(initial_box));
    let res = app.run(terminal);
    restore_tui()?;
    res.map_err(self::error::Error::from)
}

fn init_tui() -> std::io::Result<ratatui::Terminal<impl ratatui::prelude::Backend>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
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
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
