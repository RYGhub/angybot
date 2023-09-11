use std::env;
use lazy_static::lazy_static;


lazy_static! {
    /// The Discord bot token to use.
    pub static ref TOKEN: String = env::var("ANGY_TOKEN")
        .expect("ANGY_TOKEN to be set");

    /// The Discord application id to use.
    pub static ref APPID: u64 = env::var("ANGY_APPID")
        .expect("ANGY_APPID to be set")
        .parse::<u64>()
        .expect("ANGY_APPID to be a valid u64");

    /// The Plex server to use.
    pub static ref PLEX_SERVER: String = env::var("ANGY_PLEX_SERVER")
        .expect("ANGY_PLEX_SERVER to be set");

    /// The Plex token to use.
    pub static ref PLEX_TOKEN: String = env::var("ANGY_PLEX_TOKEN")
        .expect("ANGY_PLEX_TOKEN to be set");
}
