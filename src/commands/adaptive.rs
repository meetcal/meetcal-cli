use clap::Parser;

/// Search for Adaptive American Records for a given weight class and gender.
///
/// Examples:
///   meetcal adaptive --gender Men
///   meetcal adaptive Women
#[derive(Parser)]
#[command(name = "adaptive")]
pub struct AdaptiveArgs {
    /// Gender to search for
    pub gender: String,
}

pub fn run(_args: AdaptiveArgs, _convex_url: &str) {}
