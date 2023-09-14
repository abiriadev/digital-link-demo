use std::fs::read_to_string;

use anyhow::Ok;
use indicatif::{ParallelProgressIterator, ProgressBar};
use nipper::Document;
use rayon::prelude::{
	IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GoodsList {
	products: Vec<Goods>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Goods {
	// grp_path: String,
	mdl_nm: String,
	mdl_code: String,
	goods_id: String,
	goods_nm: String,
	goods_detail_url: String,
}

#[derive(Debug)]
struct PipRequest {
	// category: String,
	// grp_path: String,
	mdl_nm: String,
	mdl_code: String,
	goods_id: String,
	goods_nm: String,
	goods_detail_url: String,
}

#[derive(Deserialize)]
struct Pip {}

#[derive(Debug)]
struct ManualRequest {
	mdl_nm: String,
	goods_id: String,
}

#[derive(Debug)]
struct Manual {
	name: String,
	href: String,
}

#[derive(Debug)]
struct Collector {
	base_url: String,
	pb: ProgressBar,
}

impl Collector {
	fn new(base_url: String) -> anyhow::Result<Self> {
		Ok(Self {
			base_url,
			pb: ProgressBar::new(0),
		})
	}

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
			.map(
				|Goods {
				     mdl_code,
				     goods_detail_url,
				     mdl_nm,
				     goods_id,
				     goods_nm,
				 }| PipRequest {
					mdl_code,
					goods_detail_url,
					mdl_nm,
					goods_id,
					goods_nm,
				},
			)
			.collect())
	}

	fn parse_manual(
		&self,
		manual_request: ManualRequest,
	) -> anyhow::Result<Vec<Manual>> {
		let manual = ureq::post(&format!(
			"{}/sec/xhr/goods/goodsManual",
			self.base_url
		))
		.set("Referer", "https://www.samsung.com/")
		.send_form(&[
			("goodsId", &manual_request.goods_id),
			("mdlNm", &manual_request.mdl_nm),
		])?
		.into_string()?;

		let document = Document::from(&manual);

		Ok(document
			.select(r"li")
			.iter()
			.map(|li| Manual {
				name: li
					.select("strong.name")
					.text()
					.trim()
					.to_string(),
				href: li
					.select("a.btn-download")
					.attr("href")
					.unwrap()
					.to_string(),
			})
			.collect())
	}

	fn fetch(&self, categories: &[&str]) -> Vec<PipRequest> {
		self.pb
			.set_length(categories.len() as u64);

		categories
			.par_iter()
			.map(|req| self.parse_goods_list(req).unwrap())
			.progress_with(self.pb.clone())
			.flat_map(IntoParallelIterator::into_par_iter)
			.collect::<Vec<_>>()
	}
}

fn main() -> anyhow::Result<()> {
	let collector = Collector::new("https://www.samsung.com".to_owned())?;

	let categories = read_to_string("./categories.txt")?;
	let categories = categories
		.trim_end()
		.split('\n')
		.collect::<Vec<_>>();

	let v = collector.fetch(&categories);

	println!("{v:#?}");

	Ok(())
}
