use crate::utils::sort::{sort_by_class, weight_class_key};
use crate::{types::records::Record, utils::convex::get_convex_response};
use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use convex::Value;
use std::collections::BTreeMap;

/// Search for Records for a given age, federation, and gender.
///
/// Examples:
///   meetcal records --age Senior --gender Men --federation USAW
#[derive(Parser)]
#[command(name = "records")]
pub struct RecordsArgs {
    /// Age group to search for
    #[arg(long, short = 'a')]
    pub age: String,

    /// Gender group to search for
    #[arg(long, short = 'g')]
    pub gender: String,

    /// IWF, USAW, USAMW, or UMWF
    #[arg(long, short = 'f')]
    pub federation: String,
}

pub async fn run(args: RecordsArgs) -> Result<()> {
    let age = args.age.to_ascii_lowercase();
    let gender = args.gender.to_ascii_lowercase();
    let federation = args.federation.to_ascii_uppercase();

    let mut query_args = BTreeMap::new();

    //insert args to map
    query_args.insert("ageCategory".to_string(), Value::from(age));
    query_args.insert("gender".to_string(), Value::from(gender));
    query_args.insert("recordType".to_string(), Value::from(federation));

    let parsed_convex_result: Vec<Record> =
        get_convex_response("records:getByFederation", query_args).await?;

    let sorted = sort_by_class(parsed_convex_result, |r| r.weight_class.as_str());

    let mut table = Table::new();
    table.set_header(vec!["Class", "Snatch", "CJ", "Total"]);

    for record in sorted {
        table.add_row(vec![
            record.weight_class,
            record.snatch_record.to_string(),
            record.cj_record.to_string(),
            record.total_record.to_string(),
        ]);
    }

    println!("{table}");

    Ok(())
}
