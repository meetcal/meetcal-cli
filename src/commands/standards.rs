use crate::commands::convex::get_convex_response;
use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use convex::Value;
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

pub async fn run(args: StandardsArgs) -> Result<()> {
    // assign args to vars
    let age = args.age.to_ascii_lowercase();
    let gender = args.gender.to_ascii_lowercase();

    let mut query_args = BTreeMap::new();

    //insert args to map
    query_args.insert("ageCategory".to_string(), Value::from(age));
    query_args.insert("gender".to_string(), Value::from(gender));

    let mut parsed_convex_result: Vec<Standards> =
        get_convex_response("standards:getFiltered", query_args).await?;

    // sort by weight class low to high
    parsed_convex_result.sort_by(|a, b| a.standard_a.total_cmp(&b.standard_a));

    // push to table
    let mut table = Table::new();
    table.set_header(vec!["Class", "A", "B"]);

    for total in parsed_convex_result {
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
