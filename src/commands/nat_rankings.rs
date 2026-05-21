use anyhow::{Context, Result, bail};
use clap::Parser;
use comfy_table::Table;
use convex::{ConvexClient, FunctionResult, Value};
use std::collections::BTreeMap;

use crate::types::lifting_results::LiftingResults;

/// Search for National Rankings for a given weight.
///
/// Examples:
///   meetcal nat-rankings --weight-class "Open Men's 110kg"
///   meetcal nat-rankings "Junior Women's 77kg"
#[derive(Parser)]
#[command(name = "nat-rankings")]
pub struct NatRankingsArgs {
    /// Weight class to search for
    #[arg(long, short = 'c')]
    pub weight_class: String,
}

pub async fn run(args: NatRankingsArgs, convex_url: &str) -> Result<()> {
    // assign args to vars
    let class = args.weight_class;

    // get convex
    let mut convex = ConvexClient::new(convex_url)
        .await
        .context("Error with the convex url")?;
    let mut query_args = BTreeMap::new();

    //insert args to map
    query_args.insert("ageCategory".to_string(), Value::from(class));
    query_args.insert("federation".to_string(), Value::from("USAW"));

    let result = convex
        .query("liftingResults:getNationalRankings", query_args)
        .await?;

    // parse value convex
    let totals: Vec<LiftingResults> = match result {
        // convex returns value not string so use serde to parse
        FunctionResult::Value(val) => {
            let json_value = serde_json::Value::from(val);
            serde_json::from_value(json_value)
                .context("Failed to parse athletes from convex response")?
        }
        // bail returns error we can handle vs panic would crash and quit
        FunctionResult::ErrorMessage(err) => bail!(err),
        FunctionResult::ConvexError(err) => bail!("ConvexError: {err:?}"),
    };

    // TODO: reduce names to max value to get all unique entries
    // get list of unique athlete names filter to find those names return row of Math.max of total
    // const bestRowsByName = [...rowsByName.values()].map((row) => {
    //  return row.reduce((best, current) => {
    //    return current.total > best.total ? current : best
    //  })
    // })

    // push to table
    let mut table = Table::new();
    table.set_header(vec!["Rank", "Name", "Total"]);

    let mut rank = 1;

    for total in totals {
        table.add_row(vec![rank.to_string(), total.name, total.total.to_string()]);
        rank += 1
    }

    println!("{table}");

    Ok(())
}
