use clap::Parser;

use crate::types::athletes;

/// Analyze club performance stats for a meet.
///
/// Examples:
///   meetcal clubResults --club "POWER AND GRACE PERFORMANCE." --meet "2025 UMWF World Championships"
#[derive(Parser)]
#[command(name = "clubResults")]
pub struct ClubResultsArgs {
    /// Club name
    #[arg(long, short = 'c')]
    pub club: String,

    /// Meet name
    #[arg(long, short = 'm')]
    pub meet: String,
}

pub fn run(_args: ClubResultsArgs, _convex_url: &str) {}
