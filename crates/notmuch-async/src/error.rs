#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to send data to worker")]
    WorkerSend,
    #[error("Failed to receive data from worker")]
    WorkerRecv,

    #[error("Notmuch IO Error")]
    NotmuchIo(#[source] std::io::Error),

    #[error("Notmuch unspecified Error")]
    NotmuchUnspecified,

    #[error("Notmuch error: {}", .0)]
    Notmuch(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<notmuch::Error> for Error {
    fn from(value: notmuch::Error) -> Self {
        match value {
            notmuch::Error::IoError(error) => Self::NotmuchIo(error),
            notmuch::Error::NotmuchError(status) => Self::Notmuch(status.to_string()),
            notmuch::Error::NotmuchVerboseError(status, s) => {
                Self::Notmuch(format!("{status}: {s}"))
            }
            notmuch::Error::UnspecifiedError => Self::NotmuchUnspecified,
        }
    }
}

static_assertions::assert_impl_all!(Error: Send, Sync);
