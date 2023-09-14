use super::{catalog::CatalogParserProduct, Manual};

#[derive(Debug)]
pub struct Product {
	pub name: String,
	pub image: String,
	pub url: String,
	pub manuals: Vec<Manual>,
}

#[derive(Debug)]
pub struct ProductRequest {
	pub mdl_nm: String,
	pub goods_id: String,
}

impl From<CatalogParserProduct> for ProductRequest {
	fn from(value: CatalogParserProduct) -> Self {
		let CatalogParserProduct {
			mdl_nm,
			goods_id,
			goods_detail_url,
		} = value;

		Self { mdl_nm, goods_id }
	}
}
