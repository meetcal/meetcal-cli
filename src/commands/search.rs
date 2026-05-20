use clap::Parser;

use crate::types::lifting_results::LiftingResults;

/// Search for an athlete's name to see their Comp PRs and Results.
///
/// Examples:
///   meetcal search --name "Maddisen Mohnsen"
///   meetcal search Maddisen Mohnsen
#[derive(Parser)]
#[command(name = "search")]
pub struct SearchArgs {
    /// Athlete name to search for
    pub name: String,
}

pub fn run(_args: SearchArgs, _convex_url: &str) {}
