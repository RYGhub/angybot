//! Module defining [`AngyError`] and [`AngyResult`].


/// An error that occurred in the bot.
#[derive(Debug)]
pub enum AngyError {
    SerenityError(serenity::Error),
    SongbirdConnectionError(songbird::error::ConnectionError),
    SongbirdJoinError(songbird::error::JoinError),
    SongbirdTrackError(songbird::error::TrackError),
    UserError(&'static str),
    BotError(&'static str),
}


/// [`Result`] with [`AngyError`] as error variant.
pub type AngyResult<T> = Result<T, AngyError>;
