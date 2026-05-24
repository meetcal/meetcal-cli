use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use convex::Value;
use serde::Deserialize;
use std::collections::BTreeMap;

use crate::utils::convex::get_convex_response;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rankings {
    pub ranking: f64,
    pub name: String,
    pub weight_class: String,
    pub percent_a: f64,
    pub total: f64,
}

/// Search for International Rankings for a given age, meet, and gender.
///
/// Examples:
///   meetcal intl-rankings --age Senior --gender Men --meet Worlds
#[derive(Parser)]
#[command(name = "intl-rankings")]
pub struct IntlRankingsArgs {
    /// Age group to search for
    #[arg(long, short = 'a')]
    pub age: String,

    /// Gender group to search for
    #[arg(long, short = 'g')]
    pub gender: String,

    /// Meet to search for
    #[arg(long, short = 'm')]
    pub meet: String,
}

pub async fn run(args: IntlRankingsArgs) -> Result<()> {
    // assign args to vars
    let age = args.age;
    let gender = args.gender;
    let meet = args.meet;

    let mut query_args = BTreeMap::new();

    //insert args to map
    query_args.insert("ageCategory".to_string(), Value::from(age));
    query_args.insert("gender".to_string(), Value::from(gender));
    query_args.insert("meet".to_string(), Value::from(meet));

    let parsed_convex_result: Vec<Rankings> =
        get_convex_response("intlRankings:getFiltered", query_args).await?;

    // push to table
    let mut table = Table::new();
    table.set_header(vec!["Rank", "Name", "Class", "Percent A", "Total"]);

    for total in parsed_convex_result {
        table.add_row(vec![
            total.ranking.to_string(),
            total.name,
            total.weight_class,
            total.percent_a.to_string(),
            total.total.to_string(),
        ]);
    }

    println!("{table}");

    Ok(())
}
