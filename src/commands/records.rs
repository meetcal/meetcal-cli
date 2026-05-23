use crate::commands::convex::get_convex_response;
use crate::types::records::Record;
use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use convex::Value;
use std::collections::{BTreeMap, HashMap};

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

    // TODO: sort by weight class, find class with + and move to end
    let mut records_hash: HashMap<i32, Record> = HashMap::new();

    // go through each weight_class
    // convert to int, if it has a + add 1 so its the largest num
    for record in parsed_convex_result {
        let weight = record.weight_class.split_at(record.weight_class.len() - 2);
        let class_without_kg = weight.0;

        if class_without_kg.contains("+") {
            let weight_only = class_without_kg.split_at(class_without_kg.len() - 1);
            let num = weight_only.0.parse::<i32>().unwrap();
            let should_be_last = num + 1;
            records_hash.insert(should_be_last, record);
        } else {
            let weight_only = class_without_kg.split_at(class_without_kg.len() - 1);
            let num = weight_only.0.parse().unwrap();
            records_hash.insert(num, record);
        }
    }

    //sort by i32 key

    let mut table = Table::new();
    table.set_header(vec!["Class", "Snatch", "CJ", "Total"]);

    // for record in records {
    //     table.add_row(vec![
    //         record.weight_class,
    //         record.snatch_record.to_string(),
    //         record.cj_record.to_string(),
    //         record.total_record.to_string(),
    //     ]);
    // }

    println!("{table}");

    Ok(())
}
