micronfig::config! {
    // The Discord bot token to use.
    ANGY_TOKEN: String,
    // The Discord application id to use.
    ANGY_APPID: String > u64,

    // The Plex server to use.
    ANGY_PLEX_SERVER: String,
    // The Plex token to use.
    ANGY_PLEX_TOKEN: String,
    // The Plex library to use.
    ANGY_PLEX_LIBRARY: String,
    // The string to remove from the media file path for Plex files.
    ANGY_PLEX_REPLACE_FROM: String,
    // The string to add in the media file path for Plex files.
    ANGY_PLEX_REPLACE_TO: String,

    // The guild id to register Slash Commands in.
    ANGY_DEV_GUILD_ID: String > u64 -> serenity::model::id::GuildId,
    // The user id allowed to use /play file.
    ANGY_DEV_USER_ID: String > u64 -> serenity::model::id::UserId,
}
