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
	Path(linktype): Path<String>,
	Path(pid): Path<String>,
) -> impl IntoResponse {
	if let Some(link_type) = LINK_TYPE
		.get()
		.unwrap()
		.get(linktype.as_str())
	{
		let link_type = link_type.get("en").unwrap();

		let ctx = ProductTemplate {
			host,
			pid,
			link_type,
		};

		Html(ctx.render_once().unwrap())
	} else {
		todo!()
	}
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
