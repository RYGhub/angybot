//! Module defining the [`EventHandler`] of the bot, [`AngyHandler`].

use std::env;
use std::ffi::OsString;
use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::model::id::{GuildId, UserId};
use crate::error::{AngyError, AngyResult};
use crate::options::{option_optional_string, option_required_channel, option_required_string, options_hashmap, OptionsHashMap};
use crate::{config, plex, schema};


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
                    "summon" => summon(&ctx, cmd_guild, cmd_member, cmd_opts).await,
                    "play" => play(&ctx, cmd_guild, cmd_member, cmd_opts).await,
                    "stop" => stop(&ctx, cmd_guild, cmd_member, cmd_opts).await,
                    _ => unknown(&ctx, cmd_name, cmd_guild, cmd_member, cmd_opts).await,
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
            _ => {}
        }
    }
}


async fn summon(ctx: &Context, guild: &GuildId, _member: &Member, opts: OptionsHashMap) -> AngyResult<String> {
    debug!("Parsing options...");

    let channel = option_required_channel(&opts, "channel")?;

    debug!("Joining channel...");

    let nest = songbird::get(ctx).await.expect("songbird to be registered on the client");
    let (bird, result) = nest.join(*guild, channel.id).await;

    result.map_err(AngyError::Songbird)?;

    debug!("Joined channel successfully!");

    debug!("Deafening self...");

    let mut bird = bird.lock().await;
    bird.deafen(true).await.map_err(AngyError::Songbird)?;

    debug!("Deafened successfully!");

    Ok(format!(":crystal_ball: Joined channel <#{}>!", channel.id))
}

async fn play(ctx: &Context, guild: &GuildId, member: &Member, opts: OptionsHashMap) -> AngyResult<String> {
    let what = option_required_string(&opts, "what")?;

    let nest = songbird::get(ctx).await.expect("songbird to be registered on the client");
    let bird = nest.get(*guild).ok_or(AngyError::User("Not connected to voice."))?;
    let mut bird = bird.lock().await;

    debug!("Creating audio source...");
    let audio =
        // play ytdl
        if option_optional_string(&opts, "ytdl").is_some() {
            if what.starts_with("http://") || what.starts_with("https://") {
                songbird::ytdl(what).await.map_err(AngyError::Ffmpeg)?
            }
            else {
                let what = format!("ytsearch:{what}");
                songbird::ytdl(what).await.map_err(AngyError::Ffmpeg)?
            }
        }
        // play file
        else if option_optional_string(&opts, "file").is_some() {
            let dev_user_id: UserId = env::var("ANGY_DEV_USER_ID")
                .map_err(|_| AngyError::Bot("`ANGY_DEV_USER_ID` is not set."))?
                .parse::<u64>()
                .map_err(|_| AngyError::Bot("`ANGY_DEV_USER_ID` is not a valid ID."))?
                .into();

            match member.user.id.eq(&dev_user_id) {
                false => Err(AngyError::User("This command can be used only by the bot's owner."))?,
                true => songbird::ffmpeg(OsString::from(what)).await.map_err(AngyError::Ytdl)?
            }
        }
        // play plex
        else if option_optional_string(&opts, "plex").is_some() {
            let base_url = config::PLEX_SERVER.clone();
            let token = config::PLEX_TOKEN.clone();
            let library = config::PLEX_LIBRARY.clone();
            let client = reqwest::Client::new();

            let response = client.get(
                format!("{base_url}/hubs/search/?X-Plex-Token={token}&query={what}&limit=25&includeCollections=0&includeExternalMedia=0")
            )
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(AngyError::PlexRequest)?;

            let data: plex::SearchResults = response
                .json()
                .await
                .map_err(AngyError::PlexRequest)?;

            let track_hub: plex::Hub = data.media_container.hub.into_iter().filter_map(|hub| match hub.identifier.eq("track") {
                true => Some(hub),
                false => None,
            })
                .next()
                .ok_or(AngyError::PlexResponse("Could not find 'track' hub"))?;

            let best_match: plex::Metadata = track_hub.metadata.into_iter().filter_map(|metadata| match metadata.library_section_title == library {
                true => Some(metadata),
                false => None,
            })
                .next()
                .ok_or(AngyError::User("No results for the given query."))?;

            let bm_media: plex::Media = best_match.media.into_iter().next()
                .ok_or(AngyError::PlexResponse("No media files for the best match track."))?;

            let bm_part: plex::Part = bm_media.part.into_iter().next()
                .ok_or(AngyError::PlexResponse("No parts for the best match track."))?;

            let from = config::PLEX_REPLACE_FROM.clone();
            let to = config::PLEX_REPLACE_TO.clone();
            let path = bm_part.file.replace(&from, &to);

            log::trace!("Path is: {path:?}");

            songbird::ffmpeg(OsString::from(path)).await.map_err(AngyError::Ytdl)?
        }
        // play other
        else {
            Err(AngyError::User("Unknown subcommand."))?
        };

    let track_name = audio.metadata.track.clone().or_else(|| audio.metadata.title.clone()).unwrap_or("*unknown track*".to_string());
    let track_url = audio.metadata.source_url.clone();

    let track_text = match track_url {
        None => track_name,
        Some(track_url) => format!("[{track_name}](<{track_url}>)"),
    };

    debug!("Playing audio source: {}", &track_text);
    bird.stop();
    bird.play_only_source(audio);

    Ok(format!("️:arrow_forward: Now playing: **{}**", &track_text))
}

async fn stop(ctx: &Context, guild: &GuildId, member: &Member, opts: OptionsHashMap) -> AngyResult<String> {

    let nest = songbird::get(ctx).await.expect("songbird to be registered on the client");
    let bird = nest.get(*guild).ok_or(AngyError::User("Not connected to voice."))?;
    let mut bird = bird.lock().await;

    bird.stop();

    Ok(format!(":stop_button: Stopped playing current track."))
}

async fn unknown(_ctx: &Context, name: &str, _guild: &GuildId, _member: &Member, _opts: OptionsHashMap) -> AngyResult<String> {
    warn!("Unknown command: {name}");

    Err(AngyError::Bot("Unknown command"))
}
