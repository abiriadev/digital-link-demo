use fake::Dummy;
use serde::Serialize;

use super::{link_type::LinkType, mime::Mime};

#[derive(Debug, Dummy, Serialize)]
pub struct Resource {
	url: String,
	relation: LinkType,
	titile: String,
	language: Option<String>,
	media: Option<Mime>,
	context: Option<String>,
}
