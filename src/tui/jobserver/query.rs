use std::sync::Arc;

use futures::StreamExt;

use crate::notmuch::NotmuchWorkerHandle;
use crate::tui::app::App;
use crate::tui::error::AppError;
use crate::tui::model::MBox;
use crate::tui::model::Message;
use crate::tui::model::Tag;

pub struct QueryJob {
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
            tracing::info!("Starting job: Processing notmuch query");
            let span = tracing::debug_span!("notmuch_query_processing");
            span.record("query", &query);

            let messages = notmuch
                .create_query(&query)
                .search_messages()
                .await
                .map_err(AppError::from)?;

            let messages_len = messages.len();
            span.record("message_count", messages_len);
            tracing::debug!(?query, n = messages_len, "Found messages");

            let messages = messages
                .into_iter()
                .zip(std::iter::repeat(&span))
                .map(|(message, span)| {
                    let notmuch = notmuch.clone();
                    let message_span = tracing::debug_span!(parent: span, "message_processing");
                    message_span.record("id", message.id());

                    async move {
                        tracing::trace!(parent: &message_span, ?message, "Processing message from query");
                        let tags = notmuch
                            .clone()
                            .tags_for_message(&message)
                            .await?
                            .unwrap_or_default();

                        message_span.record("tags", tracing::field::debug(&tags));
                        tracing::trace!(parent: &message_span, ?tags, "Found tags");

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
                        message_span.record("from", &from);

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
                        message_span.record("subject", &subject);

                        tracing::debug!(id = message.id(), "Constructed message");
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
                    tracing::trace!(i, "Reporting progress");
                    if let Err(error) = state_sender
                        .blocking_send(QueryJobState::Progress((messages_len / 100 * i) as u8))
                    {
                        tracing::warn!(?error, "Failed to send progress to channel");
                    }
                })
                .map(|tpl| tpl.1)
                .collect::<Result<Vec<Message>, _>>()
                .map_err(AppError::from)?;

            tracing::debug!(parent: &span, "Constructing MBox");
            let mbox = MBox::new(query, messages);

            tracing::debug!(parent: &span, "Propagating MBox");
            if let Err(error) = state_sender.send(QueryJobState::Finished(mbox)).await {
                tracing::warn!(?error, "Failed to send mbox to channel");
            }

            Result::<(), AppError>::Ok(())
        };

        tokio::task::spawn(job);

        Self {
            state_recv,
            latest_state: QueryJobState::Started,
        }
    }

    fn update(&mut self) {
        match self.state_recv.try_recv() {
            Ok(state) => self.latest_state = state,
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                tracing::warn!("Internal channel disconnected");
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
