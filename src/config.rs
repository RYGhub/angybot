use std::env;
use lazy_static::lazy_static;
use serenity::model::id::{UserId, GuildId};


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

    /// The Plex library to use.
    pub static ref PLEX_LIBRARY: String = env::var("ANGY_PLEX_LIBRARY")
        .expect("ANGY_PLEX_LIBRARY to be set");

    /// The string to remove from the media file path for Plex files.
    pub static ref PLEX_REPLACE_FROM: String = env::var("ANGY_PLEX_REPLACE_FROM")
        .expect("ANGY_PLEX_REPLACE_FROM to be set");

    /// The string to add in the media file path for Plex files.
    pub static ref PLEX_REPLACE_TO: String = env::var("ANGY_PLEX_REPLACE_TO")
        .expect("ANGY_PLEX_REPLACE_TO to be set");

    /// The guild id to register Slash Commands in.
    pub static ref DEV_GUILD_ID: GuildId = env::var("ANGY_DEV_GUILD_ID")
        .expect("ANGY_DEV_GUILD_ID to be set")
        .parse::<u64>()
        .expect("ANGY_DEV_GUILD_ID to be a valid u64")
        .into();

    /// The user id allowed to use `/play file`.
    pub static ref DEV_USER_ID: UserId = env::var("ANGY_DEV_USER_ID")
        .expect("ANGY_DEV_USER_ID to be set")
        .parse::<u64>()
        .expect("ANGY_DEV_USER_ID to be a valid u64")
        .into();
}
