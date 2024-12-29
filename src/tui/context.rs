use crate::cli::Cli;
use crate::config::Config;
use crate::notmuch::NotmuchWorkerHandle;

#[allow(unused)]
#[derive(Debug)]
pub struct TuiContext {
    pub cli: Cli,
    pub config: Config,
    pub notmuch: NotmuchWorkerHandle,
}

impl TuiContext {
    pub fn new(cli: Cli, config: Config, notmuch: NotmuchWorkerHandle) -> Self {
        Self {
            cli,
            config,
            notmuch,
        }
    }
}
