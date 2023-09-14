use serde::Deserialize;
use ureq::Response;

use super::ResolveRequest;

#[derive(Debug)]
pub struct Catalog {
	pub product_request: Vec<CatalogParserProduct>,
}

#[derive(Debug, Deserialize)]
pub struct CatalogParser {
	pub products: Vec<CatalogParserProduct>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CatalogParserProduct {
	// uid
	pub mdl_code: String,
	// required for manual
	pub mdl_nm: String,
	// required for manual
	pub goods_id: String,
	// human readable name
	pub goods_nm: String,
	// for pip
	pub goods_detail_url: String,
	// sample image
	pub img_path1: String,
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
