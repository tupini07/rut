use clap::crate_version;
use clap::Clap;

/// The cut clone with regex capabilities.
#[derive(Clap)]
#[clap(version = crate_version!())]
pub struct AppOptions {
    /// Regex expression which specifies capture groups
    #[clap(short, long)]
    pub regex: Option<String>,

    /// String template that specifies how to use the capture group
    #[clap(short, long)]
    pub template: Option<String>,

    /// Delimiter on which to cut the string
    #[clap(short, long)]
    pub delimiter: Option<String>,

    /// Fields
    #[clap(short, long)]
    pub fields: Option<String>,

    /// Characters
    #[clap(short, long)]
    pub character_range: Option<String>,

    /// Runs in debug mode
    #[clap(long)]
    pub debug: bool,
}

pub fn parse_cli_arguments() -> AppOptions {
    AppOptions::parse()
}
