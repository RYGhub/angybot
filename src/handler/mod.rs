//! The [`EventHandler`] of the bot, [`AngyHandler`].

mod commands;

use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use crate::error::{AngyError, AngyResult};
use crate::options::{options_hashmap, OptionsHashMap};
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
                let cmd_member = &command.member.clone().expect("command to be sent by someone in a guild");
                let cmd_opts = options_hashmap(command.data.options.clone());

                trace!("{cmd_opts:#?}");

                info!("Received command: {}", &cmd_name);

                let result = match *cmd_name {
                    "summon" => commands::summon(&ctx, cmd_guild, cmd_member, cmd_opts).await,
                    "play" => commands::play(&ctx, cmd_guild, cmd_member, cmd_opts).await,
                    "stop" => commands::stop(&ctx, cmd_guild).await,
                    _ => commands::unknown(cmd_name).await,
                };

                let content = match result {
                    Ok(t) => t,
                    Err(AngyError::User(t)) => format!(":warning: {}", &t),
                    Err(e) => {
                        error!("{e:?}");
                        format!(":no_entry: Unexpected error occurred:\n```rust\n{e:#?}\n```")
                    }
                };

                command.edit_original_interaction_response(&ctx.http, |r| r
                    .content(content)
                ).await.expect("to be able to update the deferred response");
            }
            _ => {
                warn!("Received unknown interaction, ignoring");
            }
        }
    }
}
