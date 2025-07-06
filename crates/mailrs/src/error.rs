#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Configuration error")]
    Config(#[from] crate::config::ConfigError),

    #[error("Internal Toktio error")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("Notmuch Worker setup failed")]
    NotmuchWorkerSetup,

    #[error(transparent)]
    Notmuch(#[from] crate::notmuch::error::Error),

    #[cfg(feature = "gui")]
    #[error("GUI errored")]
    Gui(#[from] crate::gui::error::Error),

    #[cfg(feature = "tui")]
    #[error("TUI errored")]
    Tui(#[from] crate::tui::error::Error),
}
