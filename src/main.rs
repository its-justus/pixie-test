use std::net::SocketAddr;

use axum::{routing::{get, post},
	http::StatusCode,
	response::IntoResponse,
	Json, Router,};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let app = Router::new()
		.route("/", get(root))
		.route("/register", post(register));
	
	let addr = SocketAddr::from(([127,0,0,1], 3000));
	tracing::debug!("listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn root() -> &'static str{
	"Hello, world!"
}


async fn register(Json(payload): Json<CreateUser>) -> impl IntoResponse {
	let user = User{
		id: 42,
		username: payload.username,
	};
	(StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
	username: String,
}

#[derive(Serialize)]
struct User {
	id: usize,
	username: String,
}