#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Configuration error")]
    Config(#[from] crate::config::ConfigError),

    #[error("Internal Toktio error")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Notmuch(#[from] NotmuchError),
}

#[derive(Debug, thiserror::Error)]
#[error("Notmuch errored")]
pub struct NotmuchError(#[from] notmuch::Error);
