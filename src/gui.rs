use futures::StreamExt;
use slint::ComponentHandle;
use slint::ModelRc;
use slint::VecModel;

use crate::cli::Cli;
use crate::config::Config;
use crate::notmuch::NotmuchWorkerHandle;
use crate::slint_generatedAppWindow::*;

pub async fn run(cli: Cli, config: Config, notmuch: NotmuchWorkerHandle) -> Result<(), Error> {
    let ui = AppWindow::new()?;

    let startup_query = cli.init_query.unwrap_or(config.default_query);
    tracing::debug!(?startup_query);

    let messages = notmuch
        .create_query(&startup_query)
        .search_messages()
        .await?
        .into_iter()
        .map(|message| {
            let notmuch = notmuch.clone();

            async move {
                let tags = notmuch
                    .clone()
                    .tags_for_message(&message)
                    .await?
                    .unwrap_or_default();
                tracing::info!(id = ?message.id(), ?tags, "Found message");

                Ok(Message {
                    id: slint::SharedString::from(message.id().to_string()),
                    tags: slint::ModelRc::new(slint::VecModel::from(
                        tags.into_iter()
                            .map(|t| slint::SharedString::from(t.to_string()))
                            .map(|name| Tag { name })
                            .collect::<Vec<Tag>>(),
                    )),
                })
            }
        })
        .collect::<futures::stream::FuturesUnordered<_>>()
        .collect::<Vec<Result<Message, crate::notmuch::WorkerError<_>>>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    let mbox = MBox {
        query: slint::SharedString::from(startup_query),
        messages: slint::ModelRc::new(VecModel::from(messages)),
    };

    let facade = ui.global::<Facade>();
    facade.set_mboxes(ModelRc::new(VecModel::<MBox>::from_slice(&[mbox])));

    ui.run().map_err(Error::from)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SlintPlatform(#[from] slint::PlatformError),

    #[error(transparent)]
    WorkerNotmuch(#[from] crate::notmuch::WorkerError<crate::error::NotmuchError>),
}
