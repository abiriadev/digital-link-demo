use std::{collections::HashMap, sync::OnceLock};

use axum::{
	extract::{Host, Path},
	response::{Html, IntoResponse},
	routing::get,
	Router, Server,
};
use maplit::hashmap;
use sailfish::TemplateOnce;
use tracing_subscriber::fmt::init;

static LINK_TYPE: OnceLock<
	HashMap<&'static str, HashMap<&'static str, &'static str>>,
> = OnceLock::new();

#[derive(TemplateOnce)]
#[template(path = "product.stpl")]
struct ProductTemplate {
	host: String,
	pid: String,
}

#[tokio::main]
async fn main() {
	init();

	LINK_TYPE
		.set(hashmap! {
			"gs1:pip" => hashmap! {
				"en" => "Product information",
				"ko" => "제품 정보",
			},
			"gs1:quickStartGuide" => hashmap! {
				"en" => "Quick start guide",
				"ko" => "빠른 사용법",
			},
			"gs1:whatsInTheBox" => hashmap! {
				"en" => "What’s in the box",
				"ko" => "제품 구성물 목록",
			},
			"gs1:certificationInfo" => hashmap! {
				"en" => "Certification information",
				"ko" => "인증 정보",
			},
			"gs1:support" => hashmap! {
				"en" => "Support",
				"ko" => "지원 정보",
			},
		})
		.unwrap();

	let app = Router::new()
		.route("/:pid", get(handle_product))
		.fallback(handle_404);

	Server::bind(&"0.0.0.0:3535".parse().unwrap())
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn handle_product(
	Host(host): Host,
	Path(pid): Path<String>,
) -> impl IntoResponse {
	let ctx = ProductTemplate { host, pid };

	Html(ctx.render_once().unwrap())
}

async fn handle_404() -> impl IntoResponse { "404" }
