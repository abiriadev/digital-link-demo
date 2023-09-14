use nipper::Document;
use ureq::Response;

pub trait ResolveRequest {
	type Output;

	fn call(&self, base_url: &str) -> Result<Response, ureq::Error>;

	fn parse(&self, res: Response) -> anyhow::Result<Self::Output>;

	fn resolve(&self, base_url: &str) -> anyhow::Result<Self::Output> {
		self.parse(self.call(base_url)?)
	}
}

pub mod catalog {
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
			let CatalogParser { products } =
				res.into_json::<CatalogParser>()?;

			Ok(Self::Output {
				product_request: products
					.into_iter()
					.map(CatalogParserProduct::into)
					.collect(),
			})
		}
	}
}

pub mod product {
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
}

#[derive(Debug)]
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
