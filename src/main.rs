extern crate dotenv;

use std::env;

use dotenv::dotenv;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use serde_json::Value;

mod plex;
mod discord;

async fn plex(payload: web::Json<Value>) -> Result<HttpResponse, Error> {
  plex::hook_received(payload.to_owned()).await;

  Ok(HttpResponse::Ok().finish()) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  HttpServer::new(|| {
      App::new()
        .service(web::resource("/plex").route(web::post().to(plex)))
  })
  .bind((addr(), port()))?
  .run()
  .await
}

fn addr() -> String {
  match env::var("BINDING") {
    Ok(value) => value,
    Err(_message) => String::from("127.0.0.1"),
  }
}


fn port() -> u16 {
  match env::var("PORT") {
    Ok(value) => value.parse::<u16>().unwrap(),
    Err(_message) => 8080,
  }
}
