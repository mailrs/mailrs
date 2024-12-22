use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[clap(flatten)]
    pub(crate) verbosity: clap_verbosity_flag::Verbosity,

    // Overwrite where to look for the configuration file
    #[clap(long, value_name = "FILE")]
    pub(crate) config: Option<camino::Utf8PathBuf>,

    /// The mode to start in
    #[command(subcommand)]
    pub(crate) mode: Mode,

    /// Optional initial query, overwrites the default query from configuration
    pub(crate) init_query: Option<String>,
}

#[derive(Default, Debug, clap::Subcommand)]
pub enum Mode {
    #[cfg(feature = "gui")]
    #[cfg_attr(feature = "gui", default)]
    Gui,

    #[cfg(feature = "tui")]
    #[cfg_attr(not(feature = "gui"), default)]
    Tui,

    // to be removed
    Test,
}
