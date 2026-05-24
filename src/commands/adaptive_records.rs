use std::collections::{BTreeMap, HashMap};
use std::sync::LazyLock;

use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use convex::Value;
use regex::Regex;

use crate::types::lifting_results::AdaptiveRecord;
use crate::utils::sort::sort_by_class;
use crate::{types::lifting_results::LiftingResults, utils::convex::get_convex_response};

static MEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bmen\b").unwrap());
static WOMEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bwomen\b").unwrap());
static YEAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b\d{4}\b").unwrap());
static WEIGHT_CLASS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\b(\d+\+?)kg").unwrap());

/// Search for Adaptive American Records for a given weight class and gender.
///
/// Examples:
///   meetcal adaptive-records Women
#[derive(Parser)]
#[command(name = "adaptive-records")]
pub struct AdaptiveArgs {
    /// Gender to search for
    pub gender: String,
}

pub async fn run(args: AdaptiveArgs) -> Result<()> {
    let gender = args.gender;

    let mut query_args = BTreeMap::new();

    query_args.insert("gender".to_string(), Value::from(gender.clone()));

    let parsed_convex_response: Vec<LiftingResults> =
        get_convex_response("liftingResults:getAdaptive", BTreeMap::new()).await?;

    let mut records: HashMap<String, AdaptiveRecord> = HashMap::new();

    let filtered = parsed_convex_response
        .iter()
        .filter(|g| extract_gender(g.age.as_str(), gender.as_str()))
        .filter(|y| extract_year(y.date.as_str()) >= 2025);

    for row in filtered {
        let class = extract_class(row.age.as_str()).unwrap();

        let current = records.get(&class).cloned().unwrap_or(AdaptiveRecord {
            weight_class: class.clone(),
            snatch: 0.0,
            cj: 0.0,
            total: 0.0,
        });

        records.insert(
            class.to_string(),
            AdaptiveRecord {
                weight_class: class.to_string(),
                snatch: current.snatch.max(row.snatch_best),
                cj: current.cj.max(row.cj_best),
                total: current.total.max(row.total),
            },
        );
    }

    let sorted = sort_by_class(records.into_values().collect(), |r| r.weight_class.as_str());

    let mut table = Table::new();

    table.set_header(vec!["Weight Class", "Snatch", "CJ", "Total"]);

    for record in sorted {
        table.add_row(vec![
            record.weight_class.to_string(),
            record.snatch.to_string(),
            record.cj.to_string(),
            record.total.to_string(),
        ]);
    }

    println!("{table}");

    Ok(())
}

pub fn extract_gender(age: &str, gender: &str) -> bool {
    // Age in the db is a combo of age and weight class
    // Open Women's 86kg, Master's (40-44) Men's 95kg
    if gender.eq_ignore_ascii_case("men") {
        MEN.is_match(age) && !WOMEN.is_match(age)
    } else {
        WOMEN.is_match(age)
    }
}

pub fn extract_year(date: &str) -> u32 {
    // get year from date string
    YEAR.find(date)
        .and_then(|matched| matched.as_str().parse().ok())
        .unwrap_or(0)
}

pub fn extract_class(age: &str) -> Option<String> {
    // weight class is last portion of age db column
    // get numbers before kg, including + if there
    WEIGHT_CLASS
        .captures_iter(age)
        .filter_map(|cap| {
            let matched = cap.get(1)?;
            if is_inside_parens(age, matched.start()) {
                return None;
            }
            Some(matched.as_str().to_string())
        })
        .next()
}

fn is_inside_parens(text: &str, index: usize) -> bool {
    let before = &text[..index];
    let Some(open) = before.rfind('(') else {
        return false;
    };

    !before[open..].contains(')')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_gender() {
        assert!(extract_gender("Women", "Open Women's 77kg"));
    }

    #[test]
    fn test_extract_year() {
        assert_eq!(extract_year("2026/05/01"), 2026);
        assert_eq!(extract_year("2026-05-01"), 2026);
    }

    #[test]
    fn test_extract_class() {
        assert_eq!(extract_class("Open Women's 77kg"), Some(String::from("77")));
        assert_eq!(
            extract_class("Open Men's 110+kg"),
            Some(String::from("110+"))
        );
        assert_eq!(
            extract_class("Master's (35-39) Men's 60kg"),
            Some(String::from("60"))
        );
        assert_eq!(
            extract_class("Under 13 Age Group Women's 32kg"),
            Some(String::from("32"))
        );
    }
}
