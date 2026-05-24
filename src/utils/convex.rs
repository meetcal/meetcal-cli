use anyhow::{Context, Error, Result, bail};
use convex::{ConvexClient, FunctionResult, Value};
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

pub async fn get_convex_response<T: DeserializeOwned>(
    convex_func_name: &str,
    query_args: BTreeMap<String, Value>,
) -> Result<Vec<T>, Error> {
    // context to provide better error messages without panic like expect
    let mut convex = ConvexClient::new("https://disciplined-hare-790.convex.cloud")
        .await
        .context("Error with the convex url")?;

    let convex_result = convex.query(convex_func_name, query_args).await?;

    let parsed_result: Vec<T> = match convex_result {
        // convex returns value not string so use serde to parse
        FunctionResult::Value(val) => {
            let json_value = serde_json::Value::from(val);
            serde_json::from_value(json_value).context("Failed to parse from convex response")?
        }
        // bail returns error we can handle vs panic would crash and quit
        FunctionResult::ErrorMessage(err) => bail!(err),
        FunctionResult::ConvexError(err) => bail!("ConvexError: {err:?}"),
    };

    Ok(parsed_result)
}
