//! # Angy Bot
//!
//! Music player bot for Discord supporting unusual formats and features, intended primarly for usage in the [RYG community](https://www.ryg.one/).
//!
//! ## Supported players
//!
//! Currently supports playing from:
//! - local filesystem
//! - yt-dlp
//! - local Plex instance


mod schema;
mod error;
mod options;
mod handler;
mod config;
mod plex;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use serenity::Client;
use serenity::prelude::GatewayIntents;
use songbird::SerenityInit;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    Client::builder(&config::TOKEN.as_str(), GatewayIntents::non_privileged())
        .event_handler(handler::AngyHandler)
        .application_id(*config::APPID)
        .register_songbird()
        .await
        .expect("to be able to create the Discord client")
        .start_autosharded()
        .await
        .expect("to be able to start the Discord client")
}
