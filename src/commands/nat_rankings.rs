use anyhow::{Context, Result, bail};
use clap::Parser;
use comfy_table::Table;
use convex::{ConvexClient, FunctionResult, Value};
use std::collections::{BTreeMap, HashMap};

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

    let mut rows_hash: HashMap<String, LiftingResults> = HashMap::new();

    // if not in map insert, else if name is in map, check total, compare, keep highest
    for row in totals {
        if rows_hash.contains_key(&row.name) {
            // this is the val associated with the key
            let entry = rows_hash.get_mut(&row.name).unwrap();
            if row.total > entry.total {
                *entry = row;
            }
        } else {
            rows_hash.insert(row.name.clone(), row);
        }
    }

    // pull out just the vals from the HashMap
    let mut row_array: Vec<LiftingResults> = rows_hash.into_values().collect();
    row_array.sort_by(|a, b| b.total.total_cmp(&a.total));

    // push to table
    let mut table = Table::new();
    table.set_header(vec!["Rank", "Name", "Total"]);

    let mut rank = 1;

    for row in row_array {
        table.add_row(vec![rank.to_string(), row.name, row.total.to_string()]);
        rank += 1
    }

    println!("{table}");

    Ok(())
}
