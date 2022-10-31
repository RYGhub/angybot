//! Music player bot for Discord supporting bizzare formats and features, intended primarly for usage with the
//! [RYG community](https://www.ryg.one/).

mod schema;
mod error;
mod options;
mod handler;
mod music;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::env;
use serenity::Client;
use serenity::prelude::GatewayIntents;
use songbird::SerenityInit;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    check_envvars();

    let token = env::var("ANGY_TOKEN")
        .expect("ANGY_TOKEN to be set");
    let appid = env::var("ANGY_APPID")
        .expect("ANGY_APPID to be set")
        .parse::<u64>()
        .expect("ANGY_APPID to be a valid u64");

    Client::builder(&token, GatewayIntents::non_privileged())
        .event_handler(handler::AngyHandler)
        .application_id(appid)
        .register_songbird()
        .await
        .expect("to be able to create the Discord client")
        .start_autosharded()
        .await
        .expect("to be able to start the Discord client")
}
