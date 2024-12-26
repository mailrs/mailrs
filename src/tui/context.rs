use crate::cli::Cli;
use crate::config::Config;
use crate::notmuch::NotmuchWorkerHandle;

#[allow(unused)]
#[derive(Debug)]
pub struct TuiContext {
    cli: Cli,
    config: Config,
    notmuch: NotmuchWorkerHandle,
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
