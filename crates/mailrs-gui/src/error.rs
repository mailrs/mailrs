#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SlintPlatform(#[from] slint::PlatformError),

    #[error(transparent)]
    Notmuch(#[from] notmuch_async::error::Error),
}
