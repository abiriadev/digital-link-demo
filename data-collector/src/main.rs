use std::fs::{read_to_string, write};

use anyhow::Ok;
use data_collector::collector::Collector;

fn main() -> anyhow::Result<()> {
	let collector = Collector::new("https://www.samsung.com".to_owned())?;

	let categories = read_to_string("./categories.txt")?;
	let categories = categories
		.trim_end()
		.split('\n')
		.collect::<Vec<_>>();

	let products = collector.resolve(&categories);

	write(
		"./products.json",
		serde_json::to_string(&products)?,
	)?;

	let entries = Collector::serialize(products)?;

	write("./entries.json", entries)?;

	Ok(())
}
