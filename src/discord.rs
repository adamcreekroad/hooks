use std::env;

use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://discord.com/api/v10";

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    content: String,
    tts: bool,
}

pub async fn send_message(channel_id: String, message: String) -> Result<(), reqwest::Error> {
    let url = format!("{}/channels/{}/messages", API_URL, channel_id);

    let payload = Payload {
        content: message,
        tts: false,
    };

    let client = Client::new();

    let response = client
        .post(url)
        .json(&payload)
        .header("Authorization", format!("Bot {}", env::var("DISCORD_BOT_TOKEN").unwrap()))
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::BAD_REQUEST => println!(
            "content-length:{:?} server:{:?}",
            response.headers().get(reqwest::header::CONTENT_LENGTH),
            response.headers().get(reqwest::header::SERVER),
        ),
        _ => (),
    }

    Ok(())
}
