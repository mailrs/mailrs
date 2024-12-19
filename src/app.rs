use crate::cli::Cli;
use crate::config::Config;
use crate::error::ApplicationError;
use crate::notmuch::NotmuchWorker;

pub(crate) async fn start(cli: Cli, config: Config) -> Result<(), ApplicationError> {
    let nm_database_mode = if config.notmuch.database_readonly {
        notmuch::DatabaseMode::ReadOnly
    } else {
        notmuch::DatabaseMode::ReadWrite
    };

    let (handle_sender, handle_recv) = tokio::sync::oneshot::channel();
    {
        let notmuch_database_path = config.notmuch.database_path.clone();
        let notmuch_config_path = config.notmuch.config_path.clone();
        let notmuch_profile = config.notmuch.profile.clone();

        std::thread::spawn(move || {
            let worker = NotmuchWorker::open_database(
                notmuch_database_path,
                nm_database_mode,
                notmuch_config_path,
                notmuch_profile,
            )
            .unwrap();

            let handle = worker.handle();
            handle_sender.send(handle).unwrap();
            worker.run()
        });
    }
    let handle = handle_recv
        .await
        .map_err(|_| ApplicationError::NotmuchWorkerSetup)?;

    match cli.mode {
        crate::cli::Mode::Gui => {
            let () = crate::gui::run(cli, config, handle.clone()).await?;
        }
        crate::cli::Mode::Tui => {
            let () = crate::tui::run()?;
        }

        crate::cli::Mode::Test => {
            let startup_query = cli.init_query.unwrap_or(config.default_query);
            tracing::debug!(?startup_query);

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
