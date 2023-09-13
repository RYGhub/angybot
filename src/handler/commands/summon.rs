use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::error::{AngyError, AngyResult};
use crate::options::{option_required_channel, OptionsHashMap};

pub async fn summon(ctx: &Context, guild: &GuildId, _member: &Member, opts: OptionsHashMap) -> AngyResult<String> {
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
