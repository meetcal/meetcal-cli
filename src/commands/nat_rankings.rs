use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use convex::Value;
use std::collections::{BTreeMap, HashMap};

use crate::{commands::convex::get_convex_response, types::lifting_results::LiftingResults};

/// Search for National Rankings for a given weight.
///
/// Examples:
///   meetcal nat-rankings "Junior Women's 77kg"
#[derive(Parser)]
#[command(name = "nat-rankings")]
pub struct NatRankingsArgs {
    /// Weight class to search for
    pub weight_class: String,
}

pub async fn run(args: NatRankingsArgs) -> Result<()> {
    // assign args to vars
    let class = args.weight_class;

    let mut query_args = BTreeMap::new();

    //insert args to map
    query_args.insert("ageCategory".to_string(), Value::from(class));
    query_args.insert("federation".to_string(), Value::from("USAW"));

    let parsed_convex_result: Vec<LiftingResults> =
        get_convex_response("liftingResults:getNationalRankings", query_args).await?;

    let mut rows_hash: HashMap<String, LiftingResults> = HashMap::new();

    // if not in map insert, else if name is in map, check total, compare, keep highest
    for row in parsed_convex_result {
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

#[cfg(test)]
mod test {
    use super::*;

    const JSON: &str = r#"[
        {
            "id": 1,
            "convexId": "abc123",
            "eventId": "evt_1",
            "federation": "USAW",
            "legacyId": null,
            "meet": "American Open Finals",
            "date": "2025-12-01",
            "name": "Maddisen",
            "age": "Open",
            "bodyWeight": 77.5,
            "snatch1": 60,
            "snatch2": 65,
            "snatch3": 65,
            "snatchBest": 65,
            "cj1": 70,
            "cj2": 75,
            "cj3": 75,
            "cjBest": 75,
            "total": 140,
            "adaptive": false
        },
        {
            "id": 2,
            "convexId": "def456",
            "eventId": "evt_1",
            "federation": "USAW",
            "legacyId": null,
            "meet": "American Open Finals",
            "date": "2025-12-01",
            "name": "Nikki",
            "age": "Open",
            "bodyWeight": 77.5,
            "snatch1": 55,
            "snatch2": 60,
            "snatch3": 60,
            "snatchBest": 60,
            "cj1": 65,
            "cj2": 70,
            "cj3": 70,
            "cjBest": 70,
            "total": 130,
            "adaptive": false
        }
    ]"#;

    #[test]
    fn parse_convex() {
        let rows: Vec<LiftingResults> = serde_json::from_str(JSON).unwrap();

        assert_eq!(rows.len(), 2);

        let row = &rows[0];
        assert_eq!(row.name, "Maddisen");
        assert_eq!(row.total, 140.0);
        assert_eq!(row.federation, "USAW");
        assert_eq!(row.snatch_best, 65.0);
        assert_eq!(row.cj_best, 75.0);
        assert_eq!(row.adaptive, false);
    }

    #[test]
    fn sorting() {
        let mut rows: Vec<LiftingResults> = serde_json::from_str(JSON).unwrap();
        rows.sort_by(|a, b| b.total.total_cmp(&a.total));

        assert_eq!(rows[0].name, "Maddisen");
        assert_eq!(rows[0].total, 140.0);
        assert_eq!(rows[1].name, "Nikki");
        assert_eq!(rows[1].total, 130.0);
    }

    #[test]
    fn rejects_missing_field() {
        let bad_json = r#"[
            {
                "name": "Maddisen"
            }
        ]"#;

        let result: Result<Vec<LiftingResults>, _> = serde_json::from_str(bad_json);
        assert!(result.is_err());
    }
}
