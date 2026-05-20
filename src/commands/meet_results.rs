use clap::Parser;

use crate::types::lifting_results::LiftingResults;

/// Search for results from a meet, returns all athletes' results and event stats.
///
/// Examples:
///   meetcal meetResults --name "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness"
///   meetcal meetResults "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness"
#[derive(Parser)]
#[command(name = "meetResults")]
pub struct MeetResultsArgs {
    /// Meet to search for
    pub name: String,
}

pub fn run(_args: MeetResultsArgs, _convex_url: &str) {}
