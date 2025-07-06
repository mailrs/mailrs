use camino::Utf8PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub default_query: String,
    pub notmuch: Notmuch,
}

#[derive(Debug, serde::Deserialize)]
pub struct Notmuch {
    pub database_path: Option<Utf8PathBuf>,
    pub database_readonly: bool,
    pub config_path: Option<Utf8PathBuf>,
    pub profile: Option<String>,
}

impl Config {
    pub async fn find(overwrite: Option<Utf8PathBuf>) -> Result<Config, ConfigError> {
        let path = overwrite
            .map(Ok)
            .unwrap_or_else(find_config_path_from_xdg)?;
        if !path.exists() {
            return Err(ConfigError::DoesNotExist(path.clone()));
        }
        let s = tokio::fs::read_to_string(path).await?;
        toml::from_str(&s).map_err(ConfigError::Toml)
    }
}

fn find_config_path_from_xdg() -> Result<Utf8PathBuf, ConfigError> {
    let p = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))?
        .place_config_file("config.toml")?;
    camino::Utf8PathBuf::from_path_buf(p).map_err(ConfigError::NonUtf8Path)
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[error("Configuration file does not exist: {}", .0)]
    DoesNotExist(Utf8PathBuf),

    #[error("Non-UTF8-Path: {}", .0.display())]
    NonUtf8Path(std::path::PathBuf),

    #[error("xdg error")]
    Xdg(#[from] xdg::BaseDirectoriesError),

    #[error("toml error")]
    Toml(#[source] toml::de::Error),
}
