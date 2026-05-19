use clap::Parser;
use meetcal::commands;
use meetcal::parser::{Cli, Commands};

const CONVEX_URL: &str = "https://disciplined-hare-790.convex.cloud";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Adaptive(args) => commands::adaptive::run(args, CONVEX_URL),
        Commands::ClubResults(args) => commands::club_results::run(args, CONVEX_URL),
        Commands::IntlRankings(args) => commands::intl_rankings::run(args, CONVEX_URL),
        Commands::Meet(args) => commands::meet::run(args, CONVEX_URL),
        Commands::MeetResults(args) => commands::meet_results::run(args, CONVEX_URL),
        Commands::NatRankings(args) => commands::nat_rankings::run(args, CONVEX_URL),
        Commands::QualifyingTotals(args) => commands::qual_totals::run(args, CONVEX_URL),
        Commands::Records(args) => commands::records::run(args, CONVEX_URL),
        Commands::Search(args) => commands::search::run(args, CONVEX_URL),
        Commands::Standards(args) => commands::standards::run(args, CONVEX_URL),
        Commands::UsamwResultsScraper(args) => commands::usamw_results::run(args, CONVEX_URL),
        Commands::Wso(args) => commands::wso_results::run(args, CONVEX_URL),
        Commands::WsoOwlcms(args) => commands::wso_owlcms::run(args, CONVEX_URL),
        Commands::WsoRecords(args) => commands::wso_records::run(args, CONVEX_URL),
    }
}
