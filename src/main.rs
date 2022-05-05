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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Location {
    lat: f64,
    long: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UserLocation {
    user_id: String,
    coordinates: Location,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "{:#?}",
        reqwest::get("https://hackathlon.nitorio.us/coordinates")
            .await
            .context("Cannot fetch user information")?
            .json::<Vec<UserLocation>>()
            .await?
    );

    Ok(())
}
