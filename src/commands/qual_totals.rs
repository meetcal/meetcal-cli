use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use convex::Value;
use serde::Deserialize;
use std::collections::BTreeMap;

use crate::utils::convex::get_convex_response;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualifyingTotal {
    pub qualifying_total: f64,
    pub event_name: String,
    pub gender: String,
    pub age_category: String,
    pub weight_class: String,
}

/// Search for Qualifying Totals for a given age, gender, and event.
///
/// Examples:
///   meetcal qualifying-totals --age Senior --gender Men --event Nationals
#[derive(Parser)]
#[command(name = "qualifying-totals")]
pub struct QualTotalsArgs {
    /// Age group to search for
    #[arg(long, short = 'a')]
    pub age: String,

    /// Gender group to search for
    #[arg(long, short = 'g')]
    pub gender: String,

    /// Event to search for
    #[arg(long, short = 'e')]
    pub event: String,
}

pub async fn run(args: QualTotalsArgs) -> Result<()> {
    // assign args to vars
    let age = args.age;
    let gender = args.gender;
    let event = args.event;

    let mut query_args = BTreeMap::new();

    //insert args to map
    query_args.insert("ageCategory".to_string(), Value::from(age));
    query_args.insert("gender".to_string(), Value::from(gender));
    query_args.insert("eventName".to_string(), Value::from(event));

    let parsed_convex_result: Vec<QualifyingTotal> =
        get_convex_response("qualifyingTotals:getFiltered", query_args).await?;

    // push to table
    let mut table = Table::new();
    table.set_header(vec!["Class", "Total"]);

    for total in parsed_convex_result {
        table.add_row(vec![total.weight_class, total.qualifying_total.to_string()]);
    }

    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &str = r#"[
        {
            "qualifyingTotal": 285,
            "eventName": "Nationals",
            "gender": "Men",
            "ageCategory": "Senior",
            "weightClass": "81kg"
        }
    ]"#;

    #[test]
    fn parse_convex() {
        let totals: Vec<QualifyingTotal> = serde_json::from_str(JSON).unwrap();

        let row = &totals[0];
        assert_eq!(row.qualifying_total, 285.0);
        assert_eq!(row.event_name, "Nationals");
        assert_eq!(row.gender, "Men");
        assert_eq!(row.age_category, "Senior");
        assert_eq!(row.weight_class, "81kg");
    }

    #[test]
    fn rejects_missing_field() {
        let bad_json = r#"[{ "qualifyingTotal": 285, "gender": "Men" }]"#;
        let result: Result<Vec<QualifyingTotal>, _> = serde_json::from_str(bad_json);
        assert!(result.is_err());
    }
}
