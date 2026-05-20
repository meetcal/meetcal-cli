use crate::types::athletes::{Athletes, Platform};
use anyhow::{Context, Result, bail};
use clap::Parser;
use comfy_table::Table;
use convex::{ConvexClient, FunctionResult, Value};
use std::collections::BTreeMap;

/// Search for entries for a meet.
///
/// Examples:
///   meetcal meet --name "American Open Finals"
///   meetcal meet --name "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness" --session-number 1 --session-platform Red
///   meetcal meet "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness"
///   meetcal meet "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness" 1
///   meetcal meet "2026 Virus Weightlifting Series 2, Powered by Rogue Fitness" 1 Red
#[derive(Parser)]
#[command(name = "meet")]
pub struct MeetArgs {
    /// Meet to search for
    pub name: String,

    /// Session number to search for
    #[arg(long, short = 's')]
    pub session_number: Option<f32>,

    /// Session platform to search for
    #[arg(long, short = 'p')]
    pub session_platform: Option<Platform>,
}

pub async fn run(args: MeetArgs, convex_url: &str) -> Result<()> {
    // TODO: for whatever reason num and platform are invalid to convex
    // need to update convex fn through meetcal to allow this
    let meet_name = args.name;
    // let session_number = args.session_number.map(|n| n as f64);
    // let session_platform = args.session_platform.map(|p| match p {
    //     Platform::Red => String::from("Red"),
    //     Platform::White => String::from("White"),
    //     Platform::Blue => String::from("Blue"),
    //     Platform::Stars => String::from("Stars"),
    //     Platform::Stripes => String::from("Stripes"),
    //     Platform::Rogue => String::from("Rogue"),
    // });

    // context to provide better error messages without panic like expect
    let mut convex = ConvexClient::new(convex_url)
        .await
        .context("Error with the convex url")?;
    // BTreeMap is similar to hashmap but sorted
    // little slower but sends in order convex expects
    let mut query_args = BTreeMap::new();

    // convexes Value has built in Option so if val then val else null
    // same as sending undefined would be in TS
    query_args.insert("meet".to_string(), Value::from(meet_name));
    // query_args.insert("sessionNumber".to_string(), Value::from(session_number));
    // query_args.insert("sessionPlatform".to_string(), Value::from(session_platform));

    let result = convex.query("athletes:getByMeet", query_args).await?;

    let athletes: Vec<Athletes> = match result {
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

    let mut table = Table::new();
    table.set_header(vec![
        "Name",
        "Age",
        "Gender",
        "Adaptive",
        "Club",
        "Class",
        "Entry Total",
        "Session Num",
        "Platform",
    ]);

    for athlete in athletes {
        table.add_row(vec![
            athlete.name,
            athlete.age.to_string(),
            athlete.gender,
            athlete.adaptive.to_string(),
            athlete.club,
            athlete.weight_class,
            athlete.entry_total.to_string(),
            athlete
                .session_number
                .map(|n| n.to_string())
                .unwrap_or_else(|| "Not set".to_string()),
            athlete
                .session_platform
                .map(|p| format!("{p:?}"))
                .unwrap_or_else(|| "Not set".to_string()),
        ]);
    }

    println!("{table}");

    Ok(())
}
