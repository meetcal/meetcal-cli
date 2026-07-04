use crate::utils::sort::sort_by_class;
use crate::{types::records::Record, utils::api::get_api_response};
use anyhow::Result;
use clap::Parser;
use comfy_table::Table;

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
    let age = args.age;
    let gender = args.gender;
    let federation = args.federation.to_ascii_uppercase();

    let records: Vec<Record> = get_api_response("/data/records").await?;
    let filtered = records.into_iter().filter(|row| {
        row.age_category.eq_ignore_ascii_case(&age)
            && row.gender.eq_ignore_ascii_case(&gender)
            && row.record_type.eq_ignore_ascii_case(&federation)
    });
    let sorted = sort_by_class(filtered.collect(), |r| r.weight_class.as_str());

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
