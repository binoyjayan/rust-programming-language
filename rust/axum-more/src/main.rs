use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
pub use axum_macros::debug_handler;
use model::ModelController;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use web::mw_auth;
use web::routes_login;
use web::routes_tickets;

pub use self::error::{Error, Result};

mod ctx;
mod error;
mod model;
mod web;

const LISTEN_ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mc = ModelController::new();

    // Middleware is only applicable to the routes below it i.e. routes_api
    let routes_api =
        routes_tickets::routes(mc).layer(middleware::from_fn(mw_auth::mw_require_auth));

    // Layers are executed from bottom to top
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    println!("Listening on: {}", LISTEN_ADDRESS);
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("-->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/", get(handler_index))
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// Index handler
#[debug_handler]
async fn handler_index() -> impl IntoResponse {
    "Welcome to Axum!"
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// /hello?name=Binoy
#[debug_handler]
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}

// /hello2/Binoy
#[debug_handler]
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("-->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}

// To build and run automatically on file change:
// cargo watch -q -c -w src/ -x run
