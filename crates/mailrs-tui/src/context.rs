use notmuch_async::NotmuchWorkerHandle;

#[allow(unused)]
#[derive(Debug)]
pub struct TuiContext {
    pub notmuch: NotmuchWorkerHandle,
}

impl TuiContext {
    pub fn new(notmuch: NotmuchWorkerHandle) -> Self {
        Self { notmuch }
    }
}
