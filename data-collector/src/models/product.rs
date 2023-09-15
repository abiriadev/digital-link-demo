use std::iter::once;

use check_digit_calc::Gtin;
use indicatif::ProgressBar;
use serde::Serialize;

use super::{
	catalog::CatalogParserProduct, Manual, ManualRequest, ResolveRequest,
};

#[derive(Debug, Serialize)]
pub struct Product {
	pub model_code: String,
	// maps to `goodsNm`
	pub name: String,
	// maps to `imgPath1` and base_url resolving
	pub image: String,
	// maps to `goodsDetailUrl` and base_url resolving
	pub url: String,
	pub spec: String,
	// manual resolving
	pub manuals: Vec<Manual>,
}

impl Product {
	pub fn resolve(
		CatalogParserProduct {
			mdl_code,
			mdl_nm,
			goods_id,
			goods_nm,
			goods_detail_url,
			img_path1,
		}: CatalogParserProduct,
		base_url: &str,
		pb: ProgressBar,
	) -> Self {
		let manuals = ManualRequest {
			mdl_nm: mdl_nm.clone(),
			goods_id,
		}
		.resolve(base_url);

		pb.println(format!(
			"resolved manuals for {}",
			mdl_nm
		));

		Self {
			model_code: mdl_code,
			name: goods_nm.clone(),
			image: format!("http://{img_path1}"),
			url: format!("{base_url}/sec/{goods_detail_url}"),
			spec: format!("spec: {goods_nm}"),
			manuals: manuals.unwrap_or(vec![]),
		}
	}
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ResolverEntry {
	identification_key_type: String, // "gtin",
	identification_key: String,      // "00745883713370",
	item_description: String,        // "M00420020590",
	qualifier_path: String,          // "/",
	active: bool,
	responses: Vec<ResolverEntryResponse>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ResolverEntryResponse {
	link_type: String,           // "gs1:pip",
	iana_language: String,       // "en",
	context: String,             // "us",
	mime_type: String,           // "text/html",
	link_title: String,          // "Product Info",
	target_url: String,          // "http://lansley.com/test.html",
	default_link_type: bool,     // true,
	default_iana_language: bool, // true,
	default_context: bool,       // true,
	default_mime_type: bool,     // true,
	fwqs: bool,                  // false,
	active: bool,                // true
}

impl Default for ResolverEntryResponse {
	fn default() -> Self {
		Self {
			link_type: "gs1:pip".to_owned(),   // req
			iana_language: "ko".to_owned(),    // ?
			context: "kr".to_owned(),          // ?
			mime_type: "text/html".to_owned(), // req
			link_title: Default::default(),    // req
			target_url: Default::default(),    // req
			default_link_type: false,          // req
			default_iana_language: true,
			default_context: true,
			default_mime_type: true,
			fwqs: true,
			active: true,
		}
	}
}

impl From<Product> for ResolverEntry {
	fn from(
		Product {
			model_code,
			name,
			// image,
			url,
			// spec,
			manuals,
			..
		}: Product,
	) -> Self {
		Self {
			identification_key_type: "gtin".to_owned(), // fix
			identification_key: Gtin::generate_with_seed(&model_code)
				.to_string(),
			item_description: name.clone(),
			qualifier_path: "/".to_owned(), // fix
			active: true,                   // fix
			responses: manuals
				.into_iter()
				.map(|m| m.into())
				.chain(once(ResolverEntryResponse::from_url(
					url, name,
				)))
				.collect::<Vec<_>>(), // .concat(),
		}
	}
}

impl From<Manual> for ResolverEntryResponse {
	fn from(Manual { name, href }: Manual) -> Self {
		Self {
			link_type: "gs1:instructions".to_owned(), // all manuals are instructions
			mime_type: "application/pdf".to_owned(),  // all manuals are PDF documents
			link_title: name,
			target_url: href,
			..Default::default()
		}
	}
}

impl ResolverEntryResponse {
	fn from_url(url: String, name: String) -> Self {
		Self {
			link_type: "gs1:pip".to_owned(), // product page is pip
			link_title: name,
			target_url: url,
			default_link_type: true,
			..Default::default()
		}
	}
}
