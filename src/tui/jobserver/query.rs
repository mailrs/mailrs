use std::sync::Arc;

use futures::StreamExt;
use tokio::task::JoinHandle;

use crate::notmuch::NotmuchWorkerHandle;
use crate::tui::app::App;
use crate::tui::error::AppError;
use crate::tui::model::MBox;
use crate::tui::model::Message;
use crate::tui::model::Tag;

pub struct QueryJob {
    _job: JoinHandle<Result<(), AppError>>,
    state_recv: tokio::sync::mpsc::Receiver<QueryJobState>,
    latest_state: QueryJobState,
}

enum QueryJobState {
    Started,
    Progress(u8),
    Finished(MBox),
    Dead,
}

impl QueryJob {
    pub fn new(query: String, notmuch: NotmuchWorkerHandle) -> Self {
        let (state_sender, state_recv) = tokio::sync::mpsc::channel(1);

        let job = async move {
            let messages = notmuch
                .create_query(&query)
                .search_messages()
                .await
                .map_err(AppError::from)?;

            let messages_len = messages.len();

            let messages = messages
                .into_iter()
                .map(|message| {
                    let notmuch = notmuch.clone();

                    async move {
                        let tags = notmuch
                            .clone()
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
                .enumerate()
                .inspect(|(i, _)| {
                    if let Err(error) = state_sender
                        .blocking_send(QueryJobState::Progress((messages_len / 100 * i) as u8))
                    {
                        tracing::warn!(?error, "Failed to send progress to channel");
                    }
                })
                .map(|tpl| tpl.1)
                .collect::<Result<Vec<Message>, _>>()
                .map_err(AppError::from)?;

            let mbox = MBox::new(query, messages);

            if let Err(error) = state_sender.blocking_send(QueryJobState::Finished(mbox)) {
                tracing::warn!(?error, "Failed to send mbox to channel");
            }

            Ok(())
        };

        Self {
            _job: tokio::task::spawn(job),
            state_recv,
            latest_state: QueryJobState::Started,
        }
    }

    fn update(&mut self) {
        match self.state_recv.try_recv() {
            Ok(state) => self.latest_state = state,
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                // ?
            }
        }
    }
}

impl super::Job for QueryJob {
    fn progress_state(&mut self) -> u8 {
        self.update();

        match self.latest_state {
            QueryJobState::Started => 0,
            QueryJobState::Progress(p) => p,
            QueryJobState::Finished(_) => 100,
            QueryJobState::Dead => 100,
        }
    }

    fn ready(&mut self) -> bool {
        self.update();

        matches!(self.latest_state, QueryJobState::Finished(_))
    }

    fn finalize(&mut self, app: &mut App) {
        let mbox = match self.latest_state {
            QueryJobState::Started => todo!(),
            QueryJobState::Progress(_) => todo!(),
            QueryJobState::Finished(_) => {
                let QueryJobState::Finished(mbox) =
                    std::mem::replace(&mut self.latest_state, QueryJobState::Dead)
                else {
                    panic!()
                };

                mbox
            }
            QueryJobState::Dead => {
                panic!()
            }
        };

        app.add_box(Arc::new(mbox));
        todo!()
    }
}
