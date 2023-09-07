use std::{collections::HashMap, sync::OnceLock};

use axum::{routing::get, Router, Server};
use controllers::{handle_404, handle_product};
use r#static::init_once;
use tracing_subscriber::fmt::init;

mod controllers;
mod r#static;
mod views;

static LINK_TYPE: OnceLock<
	HashMap<&'static str, HashMap<&'static str, &'static str>>,
> = OnceLock::new();

#[tokio::main]
async fn main() {
	init();
	init_once();

	let app = Router::new()
		.route("/:linktype/:pid", get(handle_product))
		.fallback(handle_404);

	Server::bind(&"0.0.0.0:3535".parse().unwrap())
		.serve(app.into_make_service())
		.await
		.unwrap();
}
