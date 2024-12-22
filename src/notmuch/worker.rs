use super::NotmuchWorkerHandle;
use super::Request;
use super::WorkerError;
use crate::error::NotmuchError;

pub type NotmuchRequestSender = tokio::sync::mpsc::Sender<Request>;
pub type NotmuchRequestReceiver = tokio::sync::mpsc::Receiver<Request>;

pub struct NotmuchWorker {
    database: notmuch::Database,
    receiver: NotmuchRequestReceiver,
    sender: NotmuchRequestSender,
}

impl NotmuchWorker {
    pub fn open_database<DP, CP>(
        database_path: Option<DP>,
        mode: notmuch::DatabaseMode,
        config_path: Option<CP>,
        profile: Option<String>,
    ) -> Result<Self, NotmuchError>
    where
        DP: AsRef<std::path::Path>,
        CP: AsRef<std::path::Path>,
    {
        let database = notmuch::Database::open_with_config(
            database_path,
            mode,
            config_path,
            profile.as_deref(),
        )?;
        let (sender, receiver) = tokio::sync::mpsc::channel(1);
        Ok(Self {
            database,
            receiver,
            sender,
        })
    }

    pub fn handle(&self) -> NotmuchWorkerHandle {
        NotmuchWorkerHandle {
            sender: self.sender.clone(),
        }
    }

    pub fn run(mut self) -> Result<(), WorkerError<()>> {
        loop {
            let Some(request) = self.receiver.blocking_recv() else {
                return Ok(());
            };

            tracing::debug!(?request, "Got request");
            let request_processing_start_time = std::time::Instant::now();

            match request {
                Request::Shutdown(sender) => {
                    sender.send(()).map_err(|_| WorkerError::Send)?;
                    break;
                }
                Request::QuerySearchMessages { query, sender } => {
                    let q = match self.database.create_query(&query) {
                        Ok(q) => q,
                        Err(e) => {
                            sender
                                .send(Err(NotmuchError::from(e)))
                                .map_err(|_| WorkerError::Send)?;
                            continue;
                        }
                    };

                    match q.search_messages() {
                        Ok(msgs) => {
                            let messages = msgs
                                .into_iter()
                                .map(|m| super::message::Message::new(m.id().to_string(), m.date()))
                                .collect();

                            sender.send(Ok(messages)).map_err(|_| WorkerError::Send)?;
                        }
                        Err(e) => {
                            sender
                                .send(Err(NotmuchError::from(e)))
                                .map_err(|_| WorkerError::Send)?;
                            continue;
                        }
                    }
                }

                Request::TagsForMessage { message_id, sender } => {
                    let tags = match self.database.find_message(&message_id) {
                        Ok(Some(msg)) => msg.tags(),
                        Ok(None) => {
                            sender.send(Ok(None)).map_err(|_| WorkerError::Send)?;
                            continue;
                        }
                        Err(e) => {
                            sender
                                .send(Err(NotmuchError::from(e)))
                                .map_err(|_| WorkerError::Send)?;
                            continue;
                        }
                    };

                    let tags = tags.map(super::tag::Tag::new).collect();
                    sender.send(Ok(Some(tags))).map_err(|_| WorkerError::Send)?;
                }
            }

            let processing_time =
                std::time::Instant::now().saturating_duration_since(request_processing_start_time);
            tracing::debug!(?processing_time, "Request processing finished");
        }

        Ok(())
    }
}
