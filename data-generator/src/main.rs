use data_generator::LinkType;
use fake::{Dummy, Fake, Faker};
use serde::Serialize;

#[derive(Debug, Dummy, Serialize)]
pub struct Resource {
	url: String,
	relation: LinkType,
	titile: String,
	language: Option<String>,
	media: Option<String>,
	context: Option<String>,
}

fn main() {
	let a: Resource = Faker.fake();

	let res = serde_json::to_string_pretty(&a).unwrap();

	// println!("{a:#?}");
	println!("{res}");
}
