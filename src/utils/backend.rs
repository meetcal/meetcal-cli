use anyhow::{Context, Result};
use serde::{Serialize, de::DeserializeOwned};

const API_BASE_URL: &str = "https://api.meetcal.app";

pub async fn get_json<T, Q>(path: &str, query: &Q) -> Result<T>
where
    T: DeserializeOwned,
    Q: Serialize + ?Sized,
{
    let response = reqwest::Client::new()
        .get(format!("{API_BASE_URL}{path}"))
        .query(query)
        .send()
        .await
        .with_context(|| format!("Failed to call MeetCal backend route {path}"))?
        .error_for_status()
        .with_context(|| format!("MeetCal backend route {path} returned an error"))?;

    response
        .json()
        .await
        .with_context(|| format!("Failed to parse MeetCal backend response from {path}"))
}
