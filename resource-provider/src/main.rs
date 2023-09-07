use std::{collections::HashMap, sync::OnceLock};

use axum::{
	extract::{Host, Path},
	response::{Html, IntoResponse},
	routing::get,
	Router, Server,
};
use sailfish::TemplateOnce;
use tracing_subscriber::fmt::init;

static LINK_TYPE: OnceLock<HashMap<&'static str, &'static str>> =
	OnceLock::new();

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
		.set({
			let mut h = HashMap::new();
			h.insert("gs1:pip", "Product information");
			h.insert(
				"gs1:quickStartGuide",
				"Quick start guide",
			);
			h.insert("gs1:whatsInTheBox", "What’s in the box");
			h.insert(
				"gs1:certificationInfo",
				"Certification information",
			);
			h.insert("gs1:support", "Support");
			h
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
