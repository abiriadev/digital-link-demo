use nipper::Document;
use serde::Serialize;
use ureq::Response;

pub mod catalog;

pub mod product;

pub trait ResolveRequest {
	type Output;

	fn call(&self, base_url: &str) -> Result<Response, ureq::Error>;

	fn parse(&self, res: Response) -> anyhow::Result<Self::Output>;

	fn resolve(&self, base_url: &str) -> anyhow::Result<Self::Output> {
		self.parse(self.call(base_url)?)
	}
}

#[derive(Debug, Serialize)]
pub struct Manual {
	pub name: String,
	pub href: String,
}

#[derive(Debug)]
pub struct ManualRequest {
	pub mdl_nm: String,
	pub goods_id: String,
}

impl ResolveRequest for ManualRequest {
	type Output = Vec<Manual>;

	fn call(&self, base_url: &str) -> Result<Response, ureq::Error> {
		ureq::post(&format!(
			"{}/sec/xhr/goods/goodsManual",
			base_url
		))
		.set("Referer", "https://www.samsung.com/")
		.send_form(&[("goodsId", &self.goods_id), ("mdlNm", &self.mdl_nm)])
	}

	fn parse(&self, res: Response) -> anyhow::Result<Self::Output> {
		Ok(Document::from(&res.into_string()?)
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
}
