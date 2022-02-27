use std::net::SocketAddr;

use axum::{
    http::Method,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer, Origin};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // creates cors policy
    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::PUT, Method::GET, Method::POST])
        .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()))
        .allow_headers(Any);

    // creates router application and registers route handlers
    let app = Router::new()
        .route("/", get(root))
        .route("/register", post(register))
        .route("/campaign", post(create_campaign))
        .layer(cors_layer);

    // binds app to a socket and starts the application
    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
    tracing::debug!("listening on {}", &addr);
    println!("Listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

// TODO: add Discord OAuth2 support
async fn register(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    // uses serde_json to deserialize the request payload and returns a json response
    let user = User {
        id: 42,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

async fn create_campaign(Json(payload): Json<CreateCampaign>) -> impl IntoResponse {
    let campaign = Campaign {
        id: 420,
        name: payload.name,
        testers: payload.testers,
    };
    (StatusCode::CREATED, Json(campaign))
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

#[derive(Deserialize)]
struct CreateCampaign {
    name: String,
    testers: usize,
}

#[derive(Serialize)]
struct Campaign {
    id: usize,
    name: String,
    testers: usize,
}
