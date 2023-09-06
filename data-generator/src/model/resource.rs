use fake::{faker::address::en::CountryCode, Dummy};
use serde::Serialize;

use super::{language::Language, link_type::LinkType, mime::Mime};

#[derive(Debug, Dummy, Serialize)]
pub struct Resource {
	url: String,
	relation: LinkType,
	titile: String,
	language: Option<Language>,
	media: Option<Mime>,

	#[dummy(faker = "CountryCode()")]
	context: Option<String>,
}
