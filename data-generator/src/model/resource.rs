use fake::{
	faker::{address::en::CountryCode, lorem::en::Sentence},
	Dummy,
};
use serde::Serialize;

use super::{language::Language, link_type::LinkType, mime::Mime};

#[derive(Debug, Dummy, Serialize)]
pub struct Resource {
	url: String,
	relation: LinkType,

	#[dummy(faker = "Sentence(5..10)")]
	titile: String,
	language: Option<Language>,
	media: Option<Mime>,

	#[dummy(faker = "CountryCode()")]
	context: Option<String>,
}
