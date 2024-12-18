use std::sync::Arc;

use tokio::sync::Mutex;

pub struct TagsForMessage<'message> {
    database: Arc<Mutex<notmuch::Database>>,
    tags: notmuch::Tags,
    _message: &'message notmuch::Message,
}

impl<'message> TagsForMessage<'message> {
    pub fn new(
        database: Arc<Mutex<notmuch::Database>>,
        message: &'message notmuch::Message,
    ) -> Self {
        Self {
            database,
            tags: message.tags(),
            _message: message,
        }
    }

    pub async fn into_vec(self) -> Vec<String> {
        // Necessary?
        let _lock = self.database.lock().await;
        self.tags.collect()
    }
}
