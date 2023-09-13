use crate::error::{AngyError, AngyResult};


pub async fn unknown(name: &str) -> AngyResult<String> {
	warn!("Unknown command: {name}");

	Err(AngyError::Bot("Unknown command"))
}
