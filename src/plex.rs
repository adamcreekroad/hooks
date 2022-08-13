use crate::discord;

use std::env;

pub async fn hook_received(payload: serde_json::Value) {
  println!("hook received: {:?}", payload);

  let event = payload["event"].as_str().unwrap();

  match event {
    "media.play" => handle_media_play(payload).await,
    _ => ()
  }
}

async fn handle_media_play(payload: serde_json::Value) {
  let message = format!("{} is now playing {}", payload["Account"]["title"], payload["Metadata"]["title"]);

  let _result = discord::send_message(env::var("PLEX_DISCORD_CHANNEL_ID").unwrap(), message).await;
}
