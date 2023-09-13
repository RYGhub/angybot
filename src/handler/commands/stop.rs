use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::error::{AngyError, AngyResult};


pub async fn stop(ctx: &Context, guild: &GuildId) -> AngyResult<String> {
	let nest = songbird::get(ctx).await.expect("songbird to be registered on the client");
	let bird = nest.get(*guild).ok_or(AngyError::User("Not connected to voice."))?;
	let mut bird = bird.lock().await;

	bird.stop();

	Ok(":stop_button: Stopped playing current track.".to_string())
}
