use anyhow::{Context, Result, bail};
use clap::Parser;
use comfy_table::Table;
use convex::{ConvexClient, FunctionResult, Value};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Standards {
    pub weight_class: String,
    pub standard_a: f64,
    pub standard_b: f64,
}

/// Search for A/B USAW Standards for a given age and gender group.
///
/// Examples:
///   meetcal standards --age Senior --gender Men
///   meetcal standards U17 Women
#[derive(Parser)]
#[command(name = "standards")]
pub struct StandardsArgs {
    /// Age group to search for
    #[arg(long, short = 'a')]
    pub age: String,

    /// Gender group to search for
    #[arg(long, short = 'g')]
    pub gender: String,
}

pub async fn run(args: StandardsArgs, convex_url: &str) -> Result<()> {
    // assign args to vars
    let age = args.age.to_ascii_lowercase();
    let gender = args.gender.to_ascii_lowercase();

    // get convex
    let mut convex = ConvexClient::new(convex_url)
        .await
        .context("Error with the convex url")?;
    let mut query_args = BTreeMap::new();

    //insert args to map
    query_args.insert("ageCategory".to_string(), Value::from(age));
    query_args.insert("gender".to_string(), Value::from(gender));

    let result = convex.query("standards:getFiltered", query_args).await?;

    // parse value convex
    let mut totals: Vec<Standards> = match result {
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

    // sort by weight class low to high
    totals.sort_by(|a, b| a.standard_a.total_cmp(&b.standard_a));

    // push to table
    let mut table = Table::new();
    table.set_header(vec!["Class", "A", "B"]);

    for total in totals {
        table.add_row(vec![
            total.weight_class,
            total.standard_a.to_string(),
            total.standard_b.to_string(),
        ]);
    }

    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &str = r#"[
        {
            "weightClass": "Senior",
            "standardA": 140,
            "standardB": 120
        },
        {
            "weightClass": "Senior",
            "standardA": 130,
            "standardB": 110
        }
    ]"#;

    #[test]
    fn parse_convex() {
        let standards: Vec<Standards> = serde_json::from_str(JSON).unwrap();

        let row = &standards[0];

        assert_eq!(row.weight_class, "Senior");
        assert_eq!(row.standard_a, 140.0);
        assert_eq!(row.standard_b, 120.0);
    }

    #[test]
    fn sorting() {
        let mut standards: Vec<Standards> = serde_json::from_str(JSON).unwrap();
        standards.sort_by(|a, b| a.standard_a.total_cmp(&b.standard_a));

        assert_eq!(standards[0].standard_a, 130.0);
    }

    #[test]
    fn rejects_missing_field() {
        let bad_json = r#"[
            {
                weightClass: "Senior",
                standardA: 140,
            }
        ]"#;

        let result: Result<Vec<Standards>, _> = serde_json::from_str(bad_json);
        assert!(result.is_err());
    }
}
