use clap::Parser;

use crate::types::records::Record;

/// Search for Records for a given age, federation, and gender.
///
/// Examples:
///   meetcal records --age Senior --gender Men --federation USAW
///   meetcal records U17 Women IWF
#[derive(Parser)]
#[command(name = "records")]
pub struct RecordsArgs {
    /// Age group to search for
    pub age: String,

    /// Gender group to search for
    pub gender: String,

    /// IWF, USAW, USAMW, or UMWF
    pub federation: String,
}

pub fn run(_args: RecordsArgs, _convex_url: &str) {}
