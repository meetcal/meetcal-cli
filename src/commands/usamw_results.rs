use clap::Parser;

/// Export USAMW PDF results as Convex lifting_results seed data.
#[derive(Parser)]
#[command(name = "usamwResultsScraper")]
pub struct UsamwResultsArgs {
    /// Meet name
    pub meet: String,

    /// Meet date in YYYY-MM-DD format
    pub date: String,

    /// Mark results as adaptive
    #[arg(long, short = 'a')]
    pub adaptive: Option<bool>,

    /// PDF URL. Repeat this flag for multiple PDFs.
    #[arg(long, short = 'p')]
    pub pdf: Vec<String>,
}

pub fn run(_args: UsamwResultsArgs, _convex_url: &str) {}
