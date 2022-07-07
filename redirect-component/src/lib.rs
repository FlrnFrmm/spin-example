use anyhow::{Result, Error};
use spin_sdk::{
	http::{Request, Response},
	http_component
};


#[http_component]
fn redirect(_: Request) -> Result<Response> {
	http::Response::builder()
		.status(http::StatusCode::PERMANENT_REDIRECT)
		.header("location", "/index.html")
		.body(None)
		.map_err(Error::from)
}
