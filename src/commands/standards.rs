use clap::Parser;

use crate::types::standards::Standards;

/// Search for A/B USAW Standards for a given age and gender group.
///
/// Examples:
///   meetcal standards --age Senior --gender Men
///   meetcal standards U17 Women
#[derive(Parser)]
#[command(name = "standards")]
pub struct StandardsArgs {
    /// Age group to search for
    pub age: String,

    /// Gender group to search for
    pub gender: String,
}

pub fn run(_args: StandardsArgs, _convex_url: &str) {}
