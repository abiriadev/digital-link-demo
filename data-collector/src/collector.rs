use anyhow::Ok;
use indicatif::{ParallelProgressIterator, ProgressBar};
use nipper::Document;
use rayon::prelude::{
	IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

use crate::models::{Goods, GoodsList, Manual, ManualRequest, PipRequest};

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
