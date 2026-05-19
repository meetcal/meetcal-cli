use clap::Parser;

/// Search for entries for a meet.
///
/// Examples:
///   meetcal meet --name "American Open Finals"
///   meetcal meet --name "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness" --session-number 1 --session-platform Red
///   meetcal meet "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness"
///   meetcal meet "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness" 1
///   meetcal meet "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness" 1 Red
#[derive(Parser)]
#[command(name = "meet")]
pub struct MeetArgs {
    /// Meet to search for
    pub name: String,

    /// Session number to search for
    #[arg(long, short = 's')]
    pub session_number: Option<u32>,

    /// Session platform to search for
    #[arg(long, short = 'p')]
    pub session_platform: Option<String>,
}

pub fn run(args: MeetArgs, convex_url: &str) {
    println!("{}", args.name);
    println!("{convex_url}");
}
