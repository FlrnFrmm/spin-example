use anyhow::{Result, Error};
use spin_sdk::{
	http::{Request, Response},
	http_component
};
use models::Todo;


#[http_component]
fn api(request: Request) -> Result<Response> {
	match request.method() {
		&http::Method::GET => get_handler(request),
		&http::Method::POST => post_handler(request),
		_ => not_found()
	}
}

fn get_handler(request: Request) -> Result<Response> {
	if let Some(path) = request.headers().get("spin-path-info") {
		return match path.to_str()? {
			"" | "/" => all_todos(),
			_ => not_found()
		}
	}
	internal_server_error()
}

fn all_todos() -> Result<Response> {
	let todos = vec![
		Todo::new("Learn Rust".to_string()),
		Todo::new("Learn Yew".to_string())
	];
	http::Response::builder()
		.status(http::StatusCode::OK)
		.header("Content-Type", "application/json")
		.body(Some(serde_json::to_string(&todos)?.into()))
		.map_err(Error::from)
}

fn post_handler(_req: Request) -> Result<Response> {
	http::Response::builder()
		.status(http::StatusCode::NOT_IMPLEMENTED)
		.body(None)
		.map_err(Error::from)
}

fn not_found() -> Result<Response> {
	http::Response::builder()
		.status(http::StatusCode::NOT_FOUND)
		.body(None)
		.map_err(Error::from)
}

fn internal_server_error() -> Result<Response> {
	http::Response::builder()
		.status(http::StatusCode::INTERNAL_SERVER_ERROR)
		.body(None)
		.map_err(Error::from)
}
