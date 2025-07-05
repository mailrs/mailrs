use std::path::PathBuf;

use crate::error::Error;
use crate::handle::NotmuchWorkerHandle;
use crate::request::Request;

#[derive(Debug)]
pub struct Message {
    id: String,
    #[allow(unused)]
    date: Option<time::OffsetDateTime>,
    worker_handle: NotmuchWorkerHandle,
}

impl Message {
    pub fn new(
        id: String,
        date: Option<time::OffsetDateTime>,
        worker_handle: NotmuchWorkerHandle,
    ) -> Self {
        Self {
            id,
            date,
            worker_handle,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    #[allow(unused)]
    pub async fn get_file_names(&self) -> Result<Option<Vec<PathBuf>>, Error> {
        super::handle::send_and_recv(
            &self.worker_handle.sender,
            Request::file_names_for_message(self.id()),
        )
        .await?
    }

    pub async fn header(&self, header: &str) -> Result<Option<String>, Error> {
        super::handle::send_and_recv(
            &self.worker_handle.sender,
            Request::header_for_message(self.id(), header),
        )
        .await?
    }

    pub async fn content(&self) -> Result<Option<String>, crate::error::Error> {
        let pathes = super::handle::send_and_recv(
            &self.worker_handle.sender,
            Request::file_names_for_message(self.id()),
        )
        .await??;

        for path in pathes.into_iter().flatten() {
            if path.exists() {
                return tokio::fs::read_to_string(path.clone())
                    .await
                    .map(Some)
                    .map_err(crate::error::Error::from);
            }
        }

        Ok(None)
    }
}
