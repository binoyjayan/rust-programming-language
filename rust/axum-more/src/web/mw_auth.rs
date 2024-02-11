use crate::{Error, Result};

use axum::middleware::Next;
use axum::body::Body;
use axum::{http::Request, response::Response};
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(
    cookies: Cookies,
	req: Request<Body>,
	next: Next,
) -> Result<Response> {
	println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    // Check for the presence of the auth token in the cookies
    let token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    token.ok_or(Error::AuthFailNoAuthToken)?;
	Ok(next.run(req).await)
}