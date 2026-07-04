use anyhow::Result;
use clap::Parser;
use comfy_table::Table;

use crate::{
    types::wso::WSORecord,
    utils::{api::get_api_response_with_query, sort::sort_by_class},
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

    let query_args = [("age_category", age), ("gender", gender), ("wso", wso)];
    let records: Vec<WSORecord> =
        get_api_response_with_query("/data/wso/records", &query_args).await?;
    let sorted = sort_by_class(records, |r| r.weight_class.as_str());

    let mut table = Table::new();
    table.set_header(vec!["Class", "Snatch", "CJ", "Total"]);

    for record in sorted {
        table.add_row(vec![
            record.weight_class.to_string(),
            record
                .snatch_record
                .map(|value| value.to_string())
                .unwrap_or_default(),
            record
                .cj_record
                .map(|value| value.to_string())
                .unwrap_or_default(),
            record
                .total_record
                .map(|value| value.to_string())
                .unwrap_or_default(),
        ]);
    }

    println!("{table}");

    Ok(())
}
