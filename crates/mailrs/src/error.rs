#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Configuration error")]
    Config(#[from] mailrs_config::ConfigError),

    #[error("Internal Toktio error")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("Notmuch Worker setup failed")]
    NotmuchWorkerSetup,

    #[error(transparent)]
    Notmuch(#[from] notmuch_async::error::Error),

    #[error("GUI errored")]
    Gui(#[from] mailrs_gui::error::Error),

    #[error("TUI errored")]
    Tui(#[from] mailrs_tui::error::Error),
}
