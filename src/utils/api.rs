use anyhow::{Context, Error, Result};
use serde::{Serialize, de::DeserializeOwned};

const DEFAULT_API_BASE_URL: &str = "https://api.meetcal.app";

pub async fn get_api_response<T>(path: &str) -> Result<Vec<T>, Error>
where
    T: DeserializeOwned,
{
    let url = format!("{}{}", DEFAULT_API_BASE_URL, path);

    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .with_context(|| format!("Failed to call MeetCal backend route {path}"))?
        .error_for_status()
        .with_context(|| format!("MeetCal backend route {path} returned an error"))?;

    response
        .json::<Vec<T>>()
        .await
        .with_context(|| format!("Failed to parse MeetCal backend response from {path}"))
}

pub async fn get_api_response_with_query<T, Q>(path: &str, query: &Q) -> Result<Vec<T>, Error>
where
    T: DeserializeOwned,
    Q: Serialize + ?Sized,
{
    let url = format!("{}{}", DEFAULT_API_BASE_URL, path);

    let response = reqwest::Client::new()
        .get(&url)
        .query(query)
        .send()
        .await
        .with_context(|| format!("Failed to call MeetCal backend route {path}"))?
        .error_for_status()
        .with_context(|| format!("MeetCal backend route {path} returned an error"))?;

    response
        .json::<Vec<T>>()
        .await
        .with_context(|| format!("Failed to parse MeetCal backend response from {path}"))
}
