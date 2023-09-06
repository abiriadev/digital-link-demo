use data_generator::LinkType;
use fake::{Dummy, Fake, Faker};

#[derive(Debug, Dummy)]
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

	println!("{a:#?}");
}
