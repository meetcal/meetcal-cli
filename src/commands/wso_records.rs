use std::collections::BTreeMap;

use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use convex::Value;

use crate::{
    types::wso::WSORecord,
    utils::{
        convex::get_convex_response,
        sort::{sort_by_class, weight_class_key},
    },
};

/// Search for WSO Records for a given age, wso, and gender.
///
/// Examples:
///   meetcal wso-records --age Senior --gender Men --wso Carolinas
///   meetcal wso-records U17 Women Carolinas
#[derive(Parser)]
#[command(name = "wso-records")]
pub struct WsoRecordsArgs {
    /// Age group to search for
    /// U11, U13, U15, U17, Junior, Senior, Masters 35, Masters 40, etc
    #[arg(long, short = 'a')]
    pub age: String,

    /// Gender group to search for
    /// Men, Women
    #[arg(long, short = 'g')]
    pub gender: String,

    /// WSO region to search for
    /// Carolina, California South, Florida, etc
    #[arg(long, short = 'w')]
    pub wso: String,
}

pub async fn run(args: WsoRecordsArgs) -> Result<()> {
    let age = args.age;
    let gender = args.gender;
    let wso = args.wso;

    let mut query_args = BTreeMap::new();

    query_args.insert("ageCategory".to_string(), Value::from(age));
    query_args.insert("gender".to_string(), Value::from(gender));
    query_args.insert("wso".to_string(), Value::from(wso));

    let parsed_convex_result: Vec<WSORecord> =
        get_convex_response("wsoRecords:getByWso", query_args).await?;
    let sorted = sort_by_class(parsed_convex_result, |r| r.weight_class.as_str());

    let mut table = Table::new();
    table.set_header(vec!["Class", "Snatch", "CJ", "Total"]);

    for record in sorted {
        table.add_row(vec![
            record.weight_class.to_string(),
            record.snatch_record.to_string(),
            record.cj_record.to_string(),
            record.total_record.to_string(),
        ]);
    }

    println!("{table}");

    Ok(())
}
