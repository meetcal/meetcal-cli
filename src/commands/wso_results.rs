use std::collections::HashMap;
use std::process;

use anyhow::{Error, Result};
use clap::Parser;

use crate::types::athletes::Athletes;
use crate::types::lifting_results::LiftingResults;
use crate::utils::api::get_api_response_with_query;

/// Get full meet results for a given WSO.
///
/// Examples:
///   meetcal wso-results "2026 Masters National Championships & National University Championships" Carolina
#[derive(Parser)]
#[command(name = "wso-results")]
pub struct WsoResultsArgs {
    /// Meet to search for
    pub meet: String,

    /// WSO to search for
    #[arg(long, short = 'w')]
    pub wso: String,
}

pub async fn run(args: WsoResultsArgs) -> Result<()> {
    let meet = args.meet;
    let wso = args.wso;

    let wso_athlete_names = get_wso_athletes(&wso, &meet).await?;

    let _results = get_lifting_results(&wso_athlete_names).await?;

    // TODO: let pr_stats = calculate_prs(&wso_athlete_names, &results, &meet);
    // TODO: compute make rate + total weight lifted from pr_stats.target_meet_rows
    // TODO: fetch liftingResults:getByMeet with meet, then calculate medals
    // TODO: print athlete / make rate / volume / PR / medal tables

    Ok(())
}

pub async fn get_wso_athletes(wso: &str, meet: &str) -> Result<Vec<String>, Error> {
    let query_args = [("meet", meet)];
    let athletes: Vec<Athletes> =
        get_api_response_with_query("/meets/athletes", &query_args).await?;
    let wso_athletes: Vec<Athletes> = athletes
        .into_iter()
        .filter(|row| row.wso.as_deref() == Some(wso))
        .collect();

    if wso_athletes.is_empty() {
        eprintln!("No athletes from the WSO in this meet");
        process::exit(1);
    }

    let wso_athlete_names: Vec<String> = wso_athletes.iter().map(|row| row.name.clone()).collect();

    Ok(wso_athlete_names)
}

pub async fn get_lifting_results(
    wso_athlete_names: &[String],
) -> Result<HashMap<String, Vec<LiftingResults>>, Error> {
    let query_args = [("names", wso_athlete_names.join(","))];
    let lifting_results: Vec<LiftingResults> =
        get_api_response_with_query("/lifting-results/by-names", &query_args).await?;

    let mut results: HashMap<String, Vec<LiftingResults>> = HashMap::new();

    for result in lifting_results {
        let name = result.name.clone();

        results.entry(name).or_default().push(result);
    }

    Ok(results)
}

pub fn is_pr(current: f64, previous: f64) -> bool {
    // TODO: take Option<f64> — no current = not a PR, no prior = PR
    current > previous
}

pub fn calculate_current_best(
    meets: &[LiftingResults],
    get: impl Fn(&LiftingResults) -> f64,
) -> Option<f64> {
    if meets.is_empty() {
        return None;
    }

    meets.iter().fold(None, |best, row| {
        let val = get(row);
        if val == 0.0 {
            return best;
        }

        match best {
            None => Some(val),
            Some(current_best) => Some(current_best.max(val)),
        }
    })
}

pub fn calculate_prs(
    wso_athlete_names: &[String],
    results: &HashMap<String, Vec<LiftingResults>>,
    _meet: &str,
) {
    // TODO: let mut pr_details = Vec::new();
    // TODO: let mut target_meet_rows = Vec::new();
    // TODO: let mut missing_names = Vec::new();

    for name in wso_athlete_names {
        // TODO: look up athlete rows from results (empty slice if missing)
        let history = results.get(name).unwrap();
        if history.is_empty() {
            return;
        }
        // TODO: current_rows = rows where row.meet == meet
        // TODO: if current_rows is empty, push name to missing_names and continue
        // TODO: extend target_meet_rows with current_rows
        // TODO: prior_rows = rows where row.meet != meet
        // TODO: current/prior snatch, cj, total via calculate_current_best
        // TODO: if is_pr for each movement, increment count and push to pr_details
    }

    // TODO: return struct with counts, pr_details, target_meet_rows, missing_names
}
