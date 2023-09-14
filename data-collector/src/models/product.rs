use super::{
	catalog::CatalogParserProduct, Manual, ManualRequest, ResolveRequest,
};

#[derive(Debug)]
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
	) -> Self {
		let manuals = ManualRequest { mdl_nm, goods_id }.resolve(base_url);

		if manuals.is_ok() {
			println!("ok: {:#?}", mdl_code);
		} else {
			println!("err: {:#?}", mdl_code);
		}

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
