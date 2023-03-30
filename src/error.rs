//! Module defining [`AngyError`] and [`AngyResult`].


/// An error that occurred in the bot.
#[derive(Debug)]
pub enum AngyError {
    Serenity(serenity::Error),
    Songbird(songbird::error::JoinError),
    Ytdl(songbird::input::error::Error),
    Ffmpeg(songbird::input::error::Error),
    User(&'static str),
    Bot(&'static str),
}


/// [`Result`] with [`AngyError`] as error variant.
pub type AngyResult<T> = Result<T, AngyError>;
