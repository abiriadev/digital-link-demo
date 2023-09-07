use axum::{
	extract::Path,
	response::{Html, IntoResponse},
	routing::get,
	Router, Server,
};
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
	init();

	let app = Router::new().route("/:pid", get(handle_product));

	Server::bind(&"0.0.0.0:3535".parse().unwrap())
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn handle_product(Path(pid): Path<String>) -> impl IntoResponse {
	Html(format!("<h1>Product: {}</h1>", pid))
}
