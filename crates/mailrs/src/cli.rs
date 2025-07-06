use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[clap(flatten)]
    pub(crate) verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,

    /// A file path to write logs to
    #[clap(long, short)]
    pub(crate) logfile: Option<camino::Utf8PathBuf>,

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
    Gui,

    #[default]
    Tui,

    // to be removed
    Test,
}
