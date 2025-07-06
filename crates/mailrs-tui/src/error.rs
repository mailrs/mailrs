#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[error("notmuch errored")]
    Notmuch(#[from] notmuch_async::error::Error),
}
