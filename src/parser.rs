use crate::commands;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "meetcal", version = "1.0.0", about = "MeetCal CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    AdaptiveRecords(commands::adaptive_records::AdaptiveArgs),
    ClubResults(commands::club_results::ClubResultsArgs),
    Compare(commands::compare::CompareArgs),
    IntlRankings(commands::intl_rankings::IntlRankingsArgs),
    Meet(commands::meet::MeetArgs),
    MeetResults(commands::meet_results::MeetResultsArgs),
    NatRankings(commands::nat_rankings::NatRankingsArgs),
    QualifyingTotals(commands::qual_totals::QualTotalsArgs),
    Records(commands::records::RecordsArgs),
    Search(commands::search::SearchArgs),
    Standards(commands::standards::StandardsArgs),
    UsamwResultsScraper(commands::usamw_results::UsamwResultsArgs),
    Wrapped(commands::wrapped::WrappedArgs),
    Wso(commands::wso_results::WsoResultsArgs),
    WsoRecords(commands::wso_records::WsoRecordsArgs),
}
