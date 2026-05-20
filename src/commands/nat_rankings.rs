use clap::Parser;

use crate::types::lifting_results::LiftingResults;

/// Search for National Rankings for a given weight.
///
/// Examples:
///   meetcal natRankings --weight-class "Open Men's 110kg"
///   meetcal natRankings "Junior Women's 77kg"
#[derive(Parser)]
#[command(name = "natRankings")]
pub struct NatRankingsArgs {
    /// Weight class to search for
    pub weight_class: String,
}

pub fn run(_args: NatRankingsArgs, _convex_url: &str) {}
