#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub default_query: String,
}

impl Config {
    pub async fn find_xdg() -> Result<Config, ConfigError> {
        let path = xdg_config_path()?;
        let s = tokio::fs::read_to_string(path).await?;
        toml::from_str(&s).map_err(ConfigError::Toml)
    }
}

fn xdg_config_path() -> Result<camino::Utf8PathBuf, ConfigError> {
    let p = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))?
        .place_config_file("config.toml")?;
    camino::Utf8PathBuf::from_path_buf(p).map_err(ConfigError::NonUtf8Path)
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[error("Non-UTF8-Path: {}", .0.display())]
    NonUtf8Path(std::path::PathBuf),

    #[error("xdg error")]
    Xdg(#[from] xdg::BaseDirectoriesError),

    #[error("toml error")]
    Toml(#[source] toml::de::Error),
}
