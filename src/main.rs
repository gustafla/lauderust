use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    id: String,
    name: String,
    email: String,
    from_flight_id: String,
    to_flight_id: String,
    image_url: String,
    activity_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Flight {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "{:#?}",
        reqwest::get("https://hackathlon.nitorio.us/me")
            .await
            .context("Cannot fetch user information")?
            .json::<User>()
            .await?
    );

    Ok(())
}
