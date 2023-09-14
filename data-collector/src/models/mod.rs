use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoodsList {
	pub products: Vec<Goods>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Goods {
	// grp_path: String,
	pub mdl_nm: String,
	pub mdl_code: String,
	pub goods_id: String,
	pub goods_nm: String,
	pub goods_detail_url: String,
}

#[derive(Debug)]
pub struct PipRequest {
	// category: String,
	// grp_path: String,
	pub mdl_nm: String,
	pub mdl_code: String,
	pub goods_id: String,
	pub goods_nm: String,
	pub goods_detail_url: String,
}

#[derive(Deserialize)]
struct Pip {}

#[derive(Debug)]
pub struct ManualRequest {
	pub mdl_nm: String,
	pub goods_id: String,
}

#[derive(Debug)]
pub struct Manual {
	pub name: String,
	pub href: String,
}
