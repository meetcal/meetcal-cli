use std::collections::BTreeMap;

use crate::{
    types::lifting_results::{LiftingResults, PRs},
    utils::convex::get_convex_response,
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

    let mut attempt_make_rate = Table::new();

    attempt_make_rate.set_header(vec!["Sn1 Make Rate", "Sn2", "Sn3", "CJ1", "CJ2", "CJ3"]);

    attempt_make_rate.add_row(vec![
        calc_make_rate_by_attempt(&parsed_convex_result, "snatch", 1),
        calc_make_rate_by_attempt(&parsed_convex_result, "snatch", 2),
        calc_make_rate_by_attempt(&parsed_convex_result, "snatch", 3),
        calc_make_rate_by_attempt(&parsed_convex_result, "cj", 1),
        calc_make_rate_by_attempt(&parsed_convex_result, "cj", 2),
        calc_make_rate_by_attempt(&parsed_convex_result, "cj", 3),
    ]);

    println!("{attempt_make_rate}");

    let mut make_rate = Table::new();

    make_rate.set_header(vec!["Snatch Make Rate", "CJ Make Rate", "Total Make Rate"]);

    make_rate.add_row(vec![
        calc_make_rate(&parsed_convex_result, "snatch"),
        calc_make_rate(&parsed_convex_result, "cj"),
        calc_make_rate(&parsed_convex_result, "both"),
    ]);

    println!("{make_rate}");

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

pub fn calc_make_rate(results: &[LiftingResults], lift: &str) -> String {
    let mut count = 0.0;
    let mut made = 0.0;

    for meet in results {
        let attempts = match lift {
            "snatch" => vec![meet.snatch1, meet.snatch2, meet.snatch3],
            "cj" => vec![meet.cj1, meet.cj2, meet.cj3],
            "both" => vec![
                meet.snatch1,
                meet.snatch2,
                meet.snatch3,
                meet.cj1,
                meet.cj2,
                meet.cj3,
            ],
            _ => return "invalid".to_string(),
        };

        for attempt in attempts {
            if attempt >= 0.0 {
                count += 1.0;
                made += 1.0;
            } else {
                count += 1.0
            }
        }
    }

    let percent = (made / count) * 100.0;
    format!("{percent:.2}%")
}

pub fn calc_make_rate_by_attempt(
    results: &[LiftingResults],
    lift: &str,
    attempt_num: u8,
) -> String {
    let mut count = 0.0;
    let mut made = 0.0;

    for meet in results {
        let attempt = match (lift, attempt_num) {
            ("snatch", 1) => meet.snatch1,
            ("snatch", 2) => meet.snatch2,
            ("snatch", 3) => meet.snatch3,
            ("cj", 1) => meet.cj1,
            ("cj", 2) => meet.cj2,
            ("cj", 3) => meet.cj3,
            _ => return "invalid".to_string(),
        };

        if attempt >= 0.0 {
            count += 1.0;
            made += 1.0;
        } else {
            count += 1.0;
        }
    }

    if count == 0.0 {
        return "N/A".to_string();
    }

    let percent = (made / count) * 100.0;
    format!("{percent:.2}%")
}
