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
    Adaptive(commands::adaptive::AdaptiveArgs),
    ClubResults(commands::club_results::ClubResultsArgs),
    IntlRankings(commands::intl_rankings::IntlRankingsArgs),
    Meet(commands::meet::MeetArgs),
    MeetResults(commands::meet_results::MeetResultsArgs),
    NatRankings(commands::nat_rankings::NatRankingsArgs),
    QualifyingTotals(commands::qual_totals::QualTotalsArgs),
    Records(commands::records::RecordsArgs),
    Search(commands::search::SearchArgs),
    Standards(commands::standards::StandardsArgs),
    UsamwResultsScraper(commands::usamw_results::UsamwResultsArgs),
    Wso(commands::wso_results::WsoResultsArgs),
    WsoOwlcms(commands::wso_owlcms::WsoOwlcmsArgs),
    WsoRecords(commands::wso_records::WsoRecordsArgs),
}
