use async_trait::async_trait;
use std::net::SocketAddr;

use axum::{
    extract::{Extension, FromRequest, RequestParts},
    http::Method,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    AddExtensionLayer, Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer, Origin};

use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_tokio_postgres=debug")
    }
    tracing_subscriber::fmt::init();

    // setup postgres connection manager
    let manager =
        PostgresConnectionManager::new_from_stringlike("host=localhost user=pixietest", NoTls)
            .unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

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
        .layer(cors_layer)
        .layer(AddExtensionLayer::new(pool));

    // binds app to a socket and starts the application
    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
    tracing::debug!("listening on {}", &addr);
    println!("Listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

async fn root() -> &'static str {
    "Hello, world!"
}

// TODO: add Discord OAuth2 support
async fn register(
    Json(payload): Json<CreateUser>,
    Extension(pool): Extension<ConnectionPool>,
) -> impl IntoResponse {
    // get a connection from the pool
    let conn = pool.get().await.map_err(internal_error)?;
    let row = conn
        .query_one(
            "insert into users (name) values ($1) returning id, name",
            &[&payload.username],
        )
        .await
        .map_err(internal_error)?;
    let user = User {
        id: row.try_get("id").map_err(internal_error)?,
        username: row.try_get("name").map_err(internal_error)?,
    };

    (StatusCode::CREATED, Json(user))
    // let user = User{
    //     id: row.try_get::<&str, String>("name").map_err(internal_error)?,
    //     username: row.try_get::<&str, String>("name").map_err(internal_error)?
    // }

    /*
    Need to figure out how to get the row to go into the User type.
    not sure how to do this, but I assume it's possible
    */
}

async fn create_campaign(Json(payload): Json<CreateCampaign>) -> impl IntoResponse {
    let campaign = Campaign {
        id: 420,
        name: payload.name,
        testers: payload.testers,
    };
    (StatusCode::CREATED, Json(campaign))
}

#[async_trait]
trait Creates<T> {
    async fn try_create(&self, pool: ConnectionPool) -> Result<T, Error>;
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[async_trait]
impl Creates<User> for CreateUser {
    async fn try_create(self, pool: ConnectionPool) -> Result<User, Error> {
        let conn = pool.get().await.map_err(internal_error)?;
        conn.query_one(
            "insert into users (name) values ($1) returning id, name",
            &[&self.username],
        )
        .await
        .map(|row| User {
            id: row.get("id"),
            username: row.get("name"),
        })
        .map_err(internal_error)
    }
}

#[derive(Serialize)]
struct User {
    id: uuid::Uuid,
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

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> String
where
    E: std::error::Error,
{
    String::from("Error saving to database")
}
