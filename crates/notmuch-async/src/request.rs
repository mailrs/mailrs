use std::path::PathBuf;

use super::message::Message;
use super::tag::Tag;
use crate::error::Error;

pub type ResultRecv<Res, Error = crate::error::Error> =
    tokio::sync::oneshot::Receiver<Result<Res, Error>>;

#[derive(Debug)]
pub enum Request {
    Shutdown(tokio::sync::oneshot::Sender<()>),
    QuerySearchMessages {
        query: String,
        sender: tokio::sync::oneshot::Sender<Result<Vec<Message>, Error>>,
    },
    TagsForMessage {
        message_id: String,
        sender: tokio::sync::oneshot::Sender<Result<Option<Vec<Tag>>, Error>>,
    },
    FileNamesForMessage {
        message_id: String,
        sender: tokio::sync::oneshot::Sender<Result<Option<Vec<PathBuf>>, Error>>,
    },
    HeaderForMessage {
        message_id: String,
        header: String,
        sender: tokio::sync::oneshot::Sender<Result<Option<String>, Error>>,
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

    pub fn file_names_for_message(message_id: &str) -> (Self, ResultRecv<Option<Vec<PathBuf>>>) {
        let (sender, recv) = tokio::sync::oneshot::channel();
        (
            Self::FileNamesForMessage {
                message_id: message_id.to_string(),
                sender,
            },
            recv,
        )
    }

    pub fn header_for_message(
        message_id: &str,
        header: &str,
    ) -> (Self, ResultRecv<Option<String>>) {
        let (sender, recv) = tokio::sync::oneshot::channel();
        (
            Self::HeaderForMessage {
                message_id: message_id.to_string(),
                header: header.to_string(),
                sender,
            },
            recv,
        )
    }
}
