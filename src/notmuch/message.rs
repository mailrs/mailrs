use std::borrow::Cow;
use std::sync::Arc;

use tokio::sync::Mutex;

pub struct Message {
    database: Arc<Mutex<notmuch::Database>>,
    inner: notmuch::Message,
}

impl Message {
    pub(super) fn new(database: Arc<Mutex<notmuch::Database>>, inner: notmuch::Message) -> Self {
        Self { database, inner }
    }

    pub async fn tags(&self) -> crate::notmuch::TagsForMessage<'_> {
        crate::notmuch::TagsForMessage::new(self.database.clone(), &self.inner)
    }

    pub async fn id(&self) -> Cow<'_, str> {
        self.inner.id() // TODO: thread safe?
    }
}
