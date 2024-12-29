use super::message::Message;
use super::tag::Tag;
use crate::error::NotmuchError;

pub type ResultRecv<Res> = tokio::sync::oneshot::Receiver<Result<Res, NotmuchError>>;

#[derive(Debug)]
pub enum Request {
    Shutdown(tokio::sync::oneshot::Sender<()>),
    QuerySearchMessages {
        query: String,
        sender: tokio::sync::oneshot::Sender<Result<Vec<Message>, NotmuchError>>,
    },
    TagsForMessage {
        message_id: String,
        sender: tokio::sync::oneshot::Sender<Result<Option<Vec<Tag>>, NotmuchError>>,
    },
}

impl Request {
    pub(super) fn shutdown() -> (Self, tokio::sync::oneshot::Receiver<()>) {
        let (sender, recv) = tokio::sync::oneshot::channel();
        (Self::Shutdown(sender), recv)
    }

    pub fn search_messages(query: String) -> (Self, ResultRecv<Vec<Message>>) {
        let (sender, recv) = tokio::sync::oneshot::channel();
        (Self::QuerySearchMessages { query, sender }, recv)
    }

    pub fn tags_for_message(message_id: &str) -> (Self, ResultRecv<Option<Vec<Tag>>>) {
        let (sender, recv) = tokio::sync::oneshot::channel();
        (
            Self::TagsForMessage {
                message_id: message_id.to_string(),
                sender,
            },
            recv,
        )
    }
}