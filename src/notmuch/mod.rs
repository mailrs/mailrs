use std::sync::Arc;

use tokio::sync::Mutex;

mod message;
mod query;
mod tags;

pub use self::message::Message;
pub use self::query::Query;
pub use self::tags::TagsForMessage;

#[derive(Clone)]
pub struct AsyncNotmuchDatabase {
    database: Arc<Mutex<notmuch::Database>>,
}

impl From<notmuch::Database> for AsyncNotmuchDatabase {
    fn from(database: notmuch::Database) -> Self {
        Self {
            database: Arc::new(Mutex::new(database)),
        }
    }
}

impl AsyncNotmuchDatabase {
    pub fn create_query<'query>(&self, query: &'query str) -> Query<'query> {
        Query::new(self.database.clone(), query)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AsyncNotmuchError {}
