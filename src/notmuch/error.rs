#[derive(Debug, thiserror::Error)]
#[error("Worker error")]
pub enum WorkerError<T> {
    Inner(T),
    Send,
    Recv,
}
