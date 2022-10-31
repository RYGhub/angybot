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
    for command in guild.get_application_commands(&ctx.http).await.map_err(AngyError::SerenityError)? {
        trace!("Deleting /{}...", command.name);
        guild.delete_application_command(&ctx.http, command.id).await.map_err(AngyError::SerenityError)?;
        trace!("Deleted /{}!", command.name);
    }
    debug!("Deleted old commands successfully!");

    debug!("Creating new commands...");

    trace!("Creating /summon...");
    guild.create_application_command(&ctx.http, |c| c
        .name("summon")
        .description("Summon the bot to a voice channel.")
        .create_option(|o| o
            .kind(CommandOptionType::Channel)
            .channel_types(&[ChannelType::Voice])
            .name("channel")
            .description("The channel the bot should be summoned to.")
            .required(true)
        )
    ).await.map_err(AngyError::SerenityError)?;
    trace!("Created /summon!");

    trace!("Creating /play...");
    guild.create_application_command(&ctx.http, |c| c
        .name("play")
        .description("Play something in voice chat.")
        .create_option(|o| o
            .kind(CommandOptionType::String)
            .name("what")
            .description("What should be played.")
            .required(true)
        )
    ).await.map_err(AngyError::SerenityError)?;
    trace!("Created /play!");

    debug!("Commands registered successfully!");
    Ok(())
}