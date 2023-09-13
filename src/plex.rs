//! This module contains minimal structs to allow [`serde`] to [`Deserialize`](serde::Deserialize) responses from the Plex private API.

#![allow(missing_docs)]

use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
#[serde(default)]
pub struct SearchResults {
	#[serde(rename = "MediaContainer")]
	pub media_container: MediaContainer
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
#[serde(default)]
pub struct MediaContainer {
	#[serde(rename = "Hub")]
	pub hub: Vec<Hub>
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
#[serde(default)]
pub struct Hub {
	#[serde(rename = "hubIdentifier")]
	pub identifier: String,

	#[serde(rename = "Metadata")]
	pub metadata: Vec<Metadata>
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
#[serde(default)]
pub struct Metadata {
	#[serde(rename = "librarySectionTitle")]
	pub library_section_title: String,

	#[serde(rename = "Media")]
	pub media: Vec<Media>
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
#[serde(default)]
pub struct Media {
	#[serde(rename = "Part")]
	pub part: Vec<Part>
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
#[serde(default)]
pub struct Part {
	pub file: String,
}
