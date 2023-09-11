use std::env;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::channel::ChannelType;
use serenity::model::id::GuildId;
use crate::error::{AngyError, AngyResult};


pub async fn register_commands(ctx: &Context) -> AngyResult<()> {
    let guild: String = env::var("ANGY_DEV_GUILD_ID")
        .expect("ANGY_DEV_GUILD_ID to be set");
    let guild: u64 = guild.parse::<u64>()
        .expect("ANGY_DEV_GUILD_ID to be a valid u64");
    let guild: GuildId = guild.into();

    debug!("Deleting old commands...");
    for command in guild.get_application_commands(&ctx.http).await.map_err(AngyError::Serenity)? {
        trace!("Deleting /{}...", command.name);
        guild.delete_application_command(&ctx.http, command.id).await.map_err(AngyError::Serenity)?;
        trace!("Deleted /{}!", command.name);
    }
    debug!("Deleted old commands successfully!");

    debug!("Creating new commands...");

    trace!("Creating /summon...");
    guild.create_application_command(&ctx.http, |c| c
        .name("summon")
        .description("Move the bot to a voice channel.")
        .create_option(|o| o
            .kind(CommandOptionType::Channel)
            .channel_types(&[ChannelType::Voice])
            .name("channel")
            .description("The channel the bot should be move to.")
            .required(true)
        )
    ).await.map_err(AngyError::Serenity)?;
    trace!("Created /summon!");

    trace!("Creating /play...");
    guild.create_application_command(&ctx.http, |c| c
        .name("play")
        .description("Play something, overriding anything that is currently being played.")
        .create_option(|o| o
            .kind(CommandOptionType::SubCommand)
            .name("ytdl")
            .description("From YouTube or another website supported by yt-dlp.")
            .create_sub_option(|o| o
                .kind(CommandOptionType::String)
                .name("what")
                .description("The URL of the audio to play.")
                .required(true)
            )
        )
        .create_option(|o| o
            .kind(CommandOptionType::SubCommand)
            .name("file")
            .description("From a file on the local computer.")
            .create_sub_option(|o| o
                .kind(CommandOptionType::String)
                .name("what")
                .description("The path of the audio to play.")
                .required(true)
            )
        )
        .create_option(|o| o
            .kind(CommandOptionType::SubCommand)
            .name("plex")
            .description("From the configured Plex media library.")
            .create_sub_option(|o| o
                .kind(CommandOptionType::String)
                .name("what")
                .description("The query to search on the configured Plex media library.")
                .required(true)
            )
        )
    ).await.map_err(AngyError::Serenity)?;
    trace!("Created /play!");

    trace!("Creating /stop...");
    guild.create_application_command(&ctx.http, |c| c
        .name("stop")
        .description("Immediately stop playing the current track.")
    ).await.map_err(AngyError::Serenity)?;
    trace!("Created /stop!");

    Ok(())
}
