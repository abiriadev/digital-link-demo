use std::fs::read_to_string;

use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::prelude::{
	IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GoodsListRequestBody {
	page: u32,
	rows: u32,
}

#[derive(Debug, Deserialize)]
struct GoodsList {
	products: Vec<Goods>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Goods {
	grp_path: String,
}

#[derive(Debug)]
struct PipRequest(String);

#[derive(Deserialize)]
struct Pip {}

#[derive(Debug)]
struct Collector {
	base_url: String,
	pb: ProgressBar,
}

impl Collector {
	fn parse_goods_list(
		&self,
		goods_list_request: &str,
	) -> anyhow::Result<Vec<PipRequest>> {
		let goods_list = ureq::post(&format!(
			"{}/sec/xhr/pf/goodsList",
			self.base_url
		))
		.set("Referer", "https://www.samsung.com/")
		.send_form(&[
			("dispClsfNo", goods_list_request),
			("page", "1"),
			("rows", "1000"),
		])?;

		self.pb.println(format!(
			"fetched category:\t{}",
			goods_list_request
		));

		let goods_list = goods_list
			.into_json::<GoodsList>()
			.unwrap();

		Ok(goods_list
			.products
			.into_iter()
			.map(|goods| PipRequest(goods.grp_path))
			.collect())
	}
}

fn main() -> anyhow::Result<()> {
	let categories = read_to_string("./categories.txt")?;
	let categories = categories
		.trim_end()
		.split('\n')
		.collect::<Vec<_>>();

	let pb = ProgressBar::new(categories.len() as u64);

	let collector = Collector {
		base_url: "https://www.samsung.com".to_owned(),
		pb: pb.clone(),
	};

	let v = categories
		.par_iter()
		.progress_with(pb)
		.flat_map(|req| {
			collector
				.parse_goods_list(req)
				.unwrap()
				.into_par_iter()
		})
		.collect::<Vec<_>>();

	println!("{v:#?}");

	Ok(())
}
