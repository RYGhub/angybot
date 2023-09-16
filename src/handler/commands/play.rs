use std::ffi::OsString;
use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::error::{AngyError, AngyResult};
use crate::options::{option_required_string, option_optional_string, OptionsHashMap};


pub async fn play(ctx: &Context, guild: &GuildId, member: &Member, opts: OptionsHashMap) -> AngyResult<String> {
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
			let dev_user_id: UserId = crate::config::DEV_USER_ID.clone();

			match member.user.id.eq(&dev_user_id) {
				false => Err(AngyError::User("This command can be used only by the bot's owner."))?,
				true => songbird::ffmpeg(OsString::from(what)).await.map_err(AngyError::Ytdl)?
			}
		}
		// play plex
		else if option_optional_string(&opts, "plex").is_some() {
			let base_url = crate::config::PLEX_SERVER.clone();
			let token = crate::config::PLEX_TOKEN.clone();
			let library = crate::config::PLEX_LIBRARY.clone();
			let client = reqwest::Client::new();

			let response = client.get(format!("{base_url}/hubs/search/"))
				.query(&[
					("X-Plex-Token", token),
					("query", what.to_string()),
					("limit", "25".to_string()),
					("includeCollections", "0".to_string()),
					("includeExternalMedia", "0".to_string()),
				])
				.header("Accept", "application/json")
				.send()
				.await
				.map_err(AngyError::PlexRequest)?;

			let data: crate::plex::SearchResults = response
				.json()
				.await
				.map_err(AngyError::PlexRequest)?;

			let track_hub: crate::plex::Hub = data.media_container.hub.into_iter().filter_map(|hub| match hub.identifier.eq("track") {
				true => Some(hub),
				false => None,
			})
				.next()
				.ok_or(AngyError::PlexResponse("Could not find 'track' hub"))?;

			let best_match: crate::plex::Metadata = track_hub.metadata.into_iter().filter_map(|metadata| match metadata.library_section_title == library {
				true => Some(metadata),
				false => None,
			})
				.next()
				.ok_or(AngyError::User("No results for the given query."))?;

			let bm_media: crate::plex::Media = best_match.media.into_iter().next()
				.ok_or(AngyError::PlexResponse("No media files for the best match track."))?;

			let bm_part: crate::plex::Part = bm_media.part.into_iter().next()
				.ok_or(AngyError::PlexResponse("No parts for the best match track."))?;

			let from = crate::config::PLEX_REPLACE_FROM.clone();
			let to = crate::config::PLEX_REPLACE_TO.clone();
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
