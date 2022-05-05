use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Flight {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
