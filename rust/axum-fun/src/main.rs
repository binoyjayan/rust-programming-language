use axum::{
    extract::Query,
    extract::State,
    http::StatusCode,
    response::AppendHeaders,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
pub use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

struct AppState {
    id: usize,
    users: Vec<User>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // state
    let state = Arc::new(Mutex::new(AppState {
        id: 1,
        users: Vec::new(),
    }));
    // build application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(polymorphic_response))
        .route("/add-user", post(create_user))
        .with_state(state);

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Welcome to Axum!"
}

// Polymorphic return types
// If the return type of the handler is different in different cases,
// convert them into response using the 'into_response()' function
// implemented by the 'IntoResponse' trait.
// curl -v http://localhost:3000/users
#[debug_handler]
async fn polymorphic_response(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(_params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let s = state.lock().expect("Failed to lock state mutex");
    if s.users.is_empty() {
        (StatusCode::NOT_FOUND, "no users found").into_response()
    } else {
        Json(s.users.clone()).into_response()
    }
}

// curl -v http://localhost:3000/add-user -H 'Content-Type: application/json' -d '{"username": "foo"}'
#[debug_handler]
async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(_params): Query<HashMap<String, String>>, // implements FromRequestParts
    // this argument tells axum to parse the request body as JSON into a 'CreateUser' type.
    // Use this as the last argument because that implements FromRequest
    Json(payload): Json<CreateUser>, // implements FromRequest
) -> impl IntoResponse {
    // Use IntoResponse as return type instead of:
    // (StatusCode, AppendHeaders<Vec<(String, String)>>, Json<User>)
    let mut s = state.lock().expect("Failed to lock state mutex");
    let user = User {
        id: s.id,
        username: payload.username,
    };
    s.id += 1;
    s.users.push(user);

    let v: Vec<(String, String)> = vec![("username".into(), "Name".into())];

    (StatusCode::CREATED, AppendHeaders(v), Json(s.users.clone()))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Clone)]
struct User {
    id: usize,
    username: String,
}
