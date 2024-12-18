use super::message::Message;
use super::tag::Tag;
use super::worker::NotmuchRequestSender;
use super::Request;
use super::WorkerError;
use crate::error::NotmuchError;

pub struct NotmuchWorkerHandle {
    pub(super) sender: NotmuchRequestSender,
}

async fn send_and_recv<Res>(
    sender: &NotmuchRequestSender,
    (request, recv): (Request, tokio::sync::oneshot::Receiver<Res>),
) -> Result<Res, WorkerError<NotmuchError>>
where
    Res: std::fmt::Debug + Send,
{
    let () = sender.send(request).await.map_err(|_| WorkerError::Send)?;
    recv.await.map_err(|_| WorkerError::Recv)
}

impl std::fmt::Debug for NotmuchWorkerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotmuchWorkerHandle")
            .finish_non_exhaustive()
    }
}

impl NotmuchWorkerHandle {
    pub async fn shutdown(self) -> Result<(), WorkerError<()>> {
        let (request, res) = Request::shutdown();
        self.sender
            .send(request)
            .await
            .map_err(|_| WorkerError::Send)?;
        res.await.map_err(|_| WorkerError::Recv)?;
        Ok(())
    }

    pub fn create_query<'query>(&self, query: &'query str) -> Query<'query> {
        Query {
            query,
            sender: self.sender.clone(),
        }
    }

    pub async fn tags_for_message(
        &self,
        message: &Message,
    ) -> Result<Option<Vec<Tag>>, WorkerError<NotmuchError>> {
        send_and_recv(&self.sender, Request::tags_for_message(message.id()))
            .await?
            .map_err(WorkerError::Inner)
    }
}

pub struct Query<'q> {
    query: &'q str,
    sender: NotmuchRequestSender,
}

impl Query<'_> {
    pub async fn search_messages(&self) -> Result<Vec<Message>, WorkerError<NotmuchError>> {
        send_and_recv(
            &self.sender,
            Request::search_messages(self.query.to_string()),
        )
        .await?
        .map_err(WorkerError::Inner)
    }
}
