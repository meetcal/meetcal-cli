use clap::Parser;

/// Search for International Rankings for a given age, meet, and gender.
///
/// Examples:
///   meetcal intlRankings --age Senior --gender Men --meet Worlds
///   meetcal intlRankings U17 Women "Pan Ams"
#[derive(Parser)]
#[command(name = "intlRankings")]
pub struct IntlRankingsArgs {
    /// Age group to search for
    pub age: String,

    /// Gender group to search for
    pub gender: String,

    /// Meet to search for
    pub meet: String,
}

pub fn run(_args: IntlRankingsArgs, _convex_url: &str) {}
