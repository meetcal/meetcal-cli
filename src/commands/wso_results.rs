use clap::Parser;

use crate::types::lifting_results::LiftingResults;
use crate::types::wso::AthleteRow;

/// Get full meet results for a given WSO.
///
/// Examples:
///   meetcal wso --meet "2026 Virus Weightlifting Finals, Powered by Rogue Fitness" --wso Carolina
///   meetcal wso "2026 Virus Weightlifting Finals, Powered by Rogue Fitness" Carolina
#[derive(Parser)]
#[command(name = "wso")]
pub struct WsoResultsArgs {
    /// Meet to search for
    pub meet: String,

    /// WSO to search for
    pub wso: String,
}

pub fn run(_args: WsoResultsArgs) {}
