//! Module defining the [`EventHandler`] of the bot, [`AngyHandler`].

use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use crate::error::AngyError::UserError;
use crate::error::{AngyError, AngyResult};
use crate::options::{option_required_channel, options_hashmap, OptionsHashMap};
use crate::schema;


/// The [`EventHandler`] of the bot.
pub struct AngyHandler;


#[serenity::async_trait]
impl EventHandler for AngyHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        debug!("Received ready event from the gateway");

        schema::register_commands(&ctx).await.expect("to be able to register commands");

        info!("{} is ready!", &ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                command.create_interaction_response(&ctx.http, |r| r
                    .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                ).await.expect("to be able to defer the response");

                let cmd_name = &command.data.name.as_str();
                let cmd_guild = &command.data.guild_id.expect("command to be sent in a guild");
                let cmd_opts = options_hashmap(command.data.options.clone());

                let result = match *cmd_name {
                    "summon" => summon(&ctx, cmd_guild, cmd_opts).await,
                    "play" => play(&ctx, cmd_guild, cmd_opts).await,
                    _ => {
                        warn!("Received unknown command: {}", &cmd_name);
                        Err(UserError("Received an unknown command."))
                    },
                };

                let content = match result {
                    Ok(t) => format!(":white_check_mark: {}", &t),
                    Err(UserError(t)) => format!(":warning: {}", &t),
                    Err(e) => {
                        error!("{e:?}");
                        format!(":no_entry: Unexpected error occurred:\n```rust\n{e:#?}\n```")
                    }
                };

                command.edit_original_interaction_response(&ctx.http, |r| r
                    .content(content)
                ).await.expect("to be able to update the deferred response");
            }
            _ => {}
        }
    }
}


async fn summon(ctx: &Context, guild: &GuildId, opts: OptionsHashMap) -> AngyResult<&'static str> {
    info!("Received summon command!");

    let channel = option_required_channel(&opts, "channel")?;

    debug!("Joining channel...");

    let nest = songbird::get(&ctx).await.expect("songbird to be registered on the client").clone();
    let (bird, result) = nest.join(guild.clone(), channel.id).await;

    result.map_err(AngyError::SongbirdJoinError)?;

    debug!("Joined channel successfully!");

    debug!("Deafening self...");

    let mut bird = bird.lock().await;
    bird.deafen(true).await.map_err(AngyError::SongbirdJoinError)?;

    debug!("Deafened successfully!");

    Ok("Joined channel!")
}

async fn play(_ctx: &Context, _guild: &GuildId, _opts: OptionsHashMap) -> AngyResult<&'static str> {
    info!("Received play command!");

    Ok("Did not do anything!")
}