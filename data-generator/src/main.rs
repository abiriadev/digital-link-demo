use data_generator::Resource;
use fake::{Fake, Faker};

fn main() {
	let a: Resource = Faker.fake();

	let res = serde_json::to_string_pretty(&a).unwrap();

	// println!("{a:#?}");
	println!("{res}");
}
