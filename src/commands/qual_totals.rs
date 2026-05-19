use clap::Parser;

/// Search for Qualifying Totals for a given age, gender, and event.
///
/// Examples:
///   meetcal qualifyingTotals --age Senior --gender Men --event Nationals
///   meetcal qualifyingTotals U17 Women AO Finals
#[derive(Parser)]
#[command(name = "qualifyingTotals")]
pub struct QualTotalsArgs {
    /// Age group to search for
    pub age: String,

    /// Gender group to search for
    pub gender: String,

    /// Event to search for
    pub event: String,
}

pub fn run(_args: QualTotalsArgs, _convex_url: &str) {}
