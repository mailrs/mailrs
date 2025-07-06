use crate::error::Error;
use crate::message::Message;
use crate::request::Request;
use crate::tag::Tag;
use crate::worker::NotmuchRequestSender;

#[derive(Clone)]
pub struct NotmuchWorkerHandle {
    pub(super) sender: NotmuchRequestSender,
}

static_assertions::assert_impl_all!(NotmuchWorkerHandle: Send, Sync);

pub(super) async fn send_and_recv<Res>(
    sender: &NotmuchRequestSender,
    (request, recv): (Request, tokio::sync::oneshot::Receiver<Res>),
) -> Result<Res, Error>
where
    Res: std::fmt::Debug + Send,
{
    let () = sender.send(request).await.map_err(|_| Error::WorkerSend)?;
    recv.await.map_err(|_| Error::WorkerRecv)
}

impl std::fmt::Debug for NotmuchWorkerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotmuchWorkerHandle")
            .finish_non_exhaustive()
    }
}

impl NotmuchWorkerHandle {
    pub async fn shutdown(self) -> Result<(), Error> {
        let (request, res) = Request::shutdown();
        self.sender
            .send(request)
            .await
            .map_err(|_| Error::WorkerSend)?;
        res.await.map_err(|_| Error::WorkerRecv)?;
        Ok(())
    }

    pub fn create_query<'query>(&self, query: &'query str) -> Query<'query> {
        Query {
            query,
            sender: self.sender.clone(),
        }
    }

    pub async fn tags_for_message(&self, message: &Message) -> Result<Option<Vec<Tag>>, Error> {
        send_and_recv(&self.sender, Request::tags_for_message(message.id())).await?
    }
}

pub struct Query<'q> {
    query: &'q str,
    sender: NotmuchRequestSender,
}

impl Query<'_> {
    pub async fn search_messages(&self) -> Result<Vec<Message>, Error> {
        send_and_recv(
            &self.sender,
            Request::search_messages(self.query.to_string()),
        )
        .await?
    }
}
