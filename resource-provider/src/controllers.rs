use axum::{
	extract::{Host, Path},
	response::{Html, IntoResponse},
};
use sailfish::TemplateOnce;

use crate::{
	views::{NotFoundTemplate, ProductTemplate},
	LINK_TYPE,
};

pub async fn handle_product(
	Host(host): Host,
	Path(pid): Path<String>,
) -> impl IntoResponse {
	let ctx = ProductTemplate { host, pid };

	Html(ctx.render_once().unwrap())
}

pub async fn handle_404() -> impl IntoResponse {
	let ctx = NotFoundTemplate {
		link_types: LINK_TYPE
			.get()
			.unwrap()
			.keys()
			.map(|s| *s)
			.collect::<Vec<_>>(),
	};

	Html(ctx.render_once().unwrap())
}
