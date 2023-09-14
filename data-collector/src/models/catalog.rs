use serde::Deserialize;
use ureq::Response;

use super::{product::ProductRequest, ResolveRequest};

#[derive(Debug)]
pub struct Catalog {
	pub product_request: Vec<ProductRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CatalogParser {
	pub products: Vec<CatalogParserProduct>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CatalogParserProduct {
	pub mdl_nm: String,
	pub goods_id: String,
	pub goods_detail_url: String,
}

#[derive(Debug)]
pub struct CatalogRequest(String);

impl From<&str> for CatalogRequest {
	fn from(value: &str) -> Self { Self(value.to_owned()) }
}

impl ResolveRequest for CatalogRequest {
	type Output = Catalog;

	fn call(&self, base_url: &str) -> Result<Response, ureq::Error> {
		ureq::post(&format!(
			"{}/sec/xhr/pf/goodsList",
			base_url
		))
		.set("Referer", "https://www.samsung.com/")
		.send_form(&[
			("dispClsfNo", &self.0),
			("page", "1"),
			("rows", "1000"),
		])
	}

	fn parse(&self, res: Response) -> anyhow::Result<Self::Output> {
		let CatalogParser { products } = res.into_json::<CatalogParser>()?;

		Ok(Self::Output {
			product_request: products
				.into_iter()
				.map(CatalogParserProduct::into)
				.collect(),
		})
	}
}
