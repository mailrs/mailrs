#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    App(#[from] AppError),

    #[error("IO Error")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[error("notmuch errored")]
    Notmuch(#[from] crate::error::NotmuchError),

    #[error("notmuch worker errored")]
    NotmuchWorker(#[from] crate::notmuch::WorkerError<crate::error::NotmuchError>),
}
