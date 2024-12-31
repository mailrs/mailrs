#![allow(dead_code)]

use std::path::PathBuf;

use super::NotmuchWorkerHandle;
use super::Request;
use super::WorkerError;

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
    pub async fn get_file_names(
        &self,
    ) -> Result<Option<Vec<PathBuf>>, super::WorkerError<crate::error::NotmuchError>> {
        super::handle::send_and_recv(
            &self.worker_handle.sender,
            Request::file_names_for_message(self.id()),
        )
        .await?
        .map_err(WorkerError::Inner)
    }

    pub async fn header(
        &self,
        header: &str,
    ) -> Result<Option<String>, super::WorkerError<crate::error::NotmuchError>> {
        super::handle::send_and_recv(
            &self.worker_handle.sender,
            Request::header_for_message(self.id(), header),
        )
        .await?
        .map_err(WorkerError::Inner)
    }
}
