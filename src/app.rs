use crate::cli::Cli;
use crate::config::Config;
use crate::error::ApplicationError;
use crate::notmuch::NotmuchWorker;

pub(crate) async fn start(cli: Cli, config: Config) -> Result<(), ApplicationError> {
    let startup_query = cli.init_query.unwrap_or(config.default_query);
    tracing::debug!(?startup_query);

    let nm_database_mode = if config.notmuch.database_readonly {
        notmuch::DatabaseMode::ReadOnly
    } else {
        notmuch::DatabaseMode::ReadWrite
    };

    let (handle_sender, handle_recv) = tokio::sync::oneshot::channel();

    std::thread::spawn(move || {
        let worker = NotmuchWorker::open_database(
            config.notmuch.database_path.as_ref(),
            nm_database_mode,
            config.notmuch.config_path.as_ref(),
            config.notmuch.profile.as_deref(),
        )
        .unwrap();

        let handle = worker.handle();
        handle_sender.send(handle).unwrap();
        worker.run()
    });

    let handle = handle_recv
        .await
        .map_err(|_| ApplicationError::NotmuchWorkerSetup)?;

    match cli.mode {
        crate::cli::Mode::Gui => {
            let ui_result = tokio::task::spawn_blocking(|| {
                crate::gui::run()
            });

            let () = ui_result.await??;
        }
        crate::cli::Mode::Tui => {
            let () = crate::tui::run()?;
        }

        crate::cli::Mode::Test => {
            let messages = handle
                .create_query(&startup_query)
                .search_messages()
                .await?;

            for message in messages {
                let tags = handle.tags_for_message(&message).await?;
                tracing::info!(id = ?message.id(), ?tags, "Found message");
            }
        }
    }

    handle.shutdown().await?;
    Ok(())
}
