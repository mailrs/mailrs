use crate::cli::Cli;
use crate::config::Config;
use crate::notmuch::NotmuchWorkerHandle;

#[derive(Debug)]
pub struct TuiContext {
    cli: Cli,
    config: Config,
    notmuch: NotmuchWorkerHandle,
    updates: tokio::sync::mpsc::Receiver<StateUpdate>,
}

impl TuiContext {
    pub fn new(cli: Cli, config: Config, notmuch: NotmuchWorkerHandle) -> Self {
        let (_sender, updates) = tokio::sync::mpsc::channel(1);
        Self {
            cli,
            config,
            notmuch,
            updates,
        }
    }

    pub async fn get_state_update(&mut self) -> Option<StateUpdate> {
        self.updates.recv().await
    }
}

pub enum StateUpdate {}
