#[derive(Debug, thiserror::Error)]
pub enum WorkerError<T> {
    #[error(transparent)]
    Inner(T),

    #[error("Failed to send with internal sender")]
    Send,

    #[error("Failed to receive with internal receiver")]
    Recv,

    #[error("File not found: {}", .path.display())]
    NoFile {
        source: std::io::Error,
        path: std::path::PathBuf,
    }
}
