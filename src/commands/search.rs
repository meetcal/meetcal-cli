use std::collections::BTreeMap;

use crate::{
    types::lifting_results::{LiftingResults, PRs},
    utils::{
        convex::get_convex_response,
        make_rate::{calc_make_rate, calc_make_rate_by_attempt, print_make_rate},
    },
};
use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use convex::Value;

/// Search for an athlete's name to see their Comp PRs and Results.
///
/// Examples:
///   meetcal search "Maddisen Mohnsen"
#[derive(Parser)]
#[command(name = "search")]
pub struct SearchArgs {
    /// Athlete name to search for
    pub name: String,
}

pub async fn run(args: SearchArgs) -> Result<()> {
    let name = args.name;

    let mut query_args = BTreeMap::new();

    query_args.insert("name".to_string(), Value::from(name));

    let parsed_convex_result: Vec<LiftingResults> =
        get_convex_response("liftingResults:getByName", query_args).await?;

    let mut meets_table = Table::new();

    meets_table.set_header(vec![
        "Meet", "Date", "Age", "Sn1", "Sn2", "Sn3", "CJ1", "CJ2", "CJ3", "Total",
    ]);

    for meet in &parsed_convex_result {
        meets_table.add_row(vec![
            meet.meet.to_string(),
            meet.date.to_string(),
            meet.age.to_string(),
            meet.snatch1.to_string(),
            meet.snatch2.to_string(),
            meet.snatch3.to_string(),
            meet.cj1.to_string(),
            meet.cj2.to_string(),
            meet.cj3.to_string(),
            meet.total.to_string(),
        ]);
    }

    println!("{meets_table}");

    let mut pr_table = Table::new();

    pr_table.set_header(vec!["Snatch PR", "CJ PR", "Total PR"]);

    let prs = get_prs(&parsed_convex_result);

    pr_table.add_row(vec![
        prs.snatch_best.to_string(),
        prs.cj_best.to_string(),
        prs.total_best.to_string(),
    ]);

    println!("{pr_table}");

    print_make_rate(&parsed_convex_result);

    Ok(())
}

pub fn get_prs(results: &[LiftingResults]) -> PRs {
    results.iter().fold(
        PRs {
            snatch_best: 0.0,
            cj_best: 0.0,
            total_best: 0.0,
        },
        |acc, meet| PRs {
            snatch_best: acc.snatch_best.max(meet.snatch_best),
            cj_best: acc.cj_best.max(meet.cj_best),
            total_best: acc.total_best.max(meet.total),
        },
    )
}
