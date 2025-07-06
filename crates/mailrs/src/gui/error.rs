#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SlintPlatform(#[from] slint::PlatformError),

    #[error(transparent)]
    WorkerNotmuch(#[from] crate::notmuch::WorkerError<crate::error::NotmuchError>),
}
