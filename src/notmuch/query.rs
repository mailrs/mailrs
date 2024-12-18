use std::sync::Arc;

use tokio::sync::Mutex;

use crate::error::NotmuchError;

pub struct Query<'query> {
    database: Arc<Mutex<notmuch::Database>>,
    query: &'query str,
}

impl<'query> Query<'query> {
    pub(super) fn new(database: Arc<Mutex<notmuch::Database>>, query: &'query str) -> Self {
        Self { database, query }
    }

    pub async fn search_messages(&self) -> Result<Vec<crate::notmuch::Message>, NotmuchError> {
        Ok(self
            .database
            .lock()
            .await
            .create_query(self.query)?
            .search_messages()?
            .map(|m| crate::notmuch::message::Message::new(self.database.clone(), m))
            .collect())
    }
}
