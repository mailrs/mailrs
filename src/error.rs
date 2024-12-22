#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Configuration error")]
    Config(#[from] crate::config::ConfigError),

    #[error("Internal Toktio error")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Notmuch(#[from] NotmuchError),

    #[error("Notmuch Worker setup failed")]
    NotmuchWorkerSetup,

    #[error("Notmuch Worker errored")]
    Worker(#[from] crate::notmuch::WorkerError<()>),

    #[error("Notmuch Worker errored")]
    WorkerNotmuch(#[from] crate::notmuch::WorkerError<NotmuchError>),

    #[error("GUI errored")]
    Gui(#[from] crate::gui::error::Error),

    #[error("TUI errored")]
    Tui(#[from] crate::tui::Error),
}

#[derive(Debug, thiserror::Error)]
#[error("Notmuch errored")]
pub struct NotmuchError(#[from] notmuch::Error);
