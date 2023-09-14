use nipper::Document;
use serde::Deserialize;

#[derive(Debug)]
struct GoodsListRequest(String);

#[derive(Debug)]
struct PipRequest(String);

#[derive(Deserialize)]
struct Pip {}

#[derive(Debug)]
struct Collector {
	base_url: String,
}

impl Collector {
	fn parse_root(&self) -> anyhow::Result<Vec<GoodsListRequest>> {
		Ok(Document::from(
			&ureq::get(&format!("{}/sec", self.base_url))
				.call()?
				.into_string()?,
		)
		.select("#footer > div:nth-child(2) > nav > ul > li")
		.iter()
		.take(2)
		.flat_map(|li| li.select("ul > li > a").iter())
		.map(|a| {
			a.attr("data-omni")
				.unwrap()
				.split_once('_')
				.unwrap()
				.1
				.to_owned()
		})
		.map(GoodsListRequest)
		.collect::<Vec<_>>())
	}
}

fn main() -> anyhow::Result<()> {
	println!("Hello, world!");

	let collector = Collector {
		base_url: "https://www.samsung.com".to_owned(),
	};

	let v = collector.parse_root()?;
	println!("{v:#?}");

	Ok(())
}
