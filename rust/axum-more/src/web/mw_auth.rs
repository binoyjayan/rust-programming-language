use crate::{Error, Result};

use axum::body::Body;
use axum::middleware::Next;
use axum::{http::Request, response::Response};
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    // Check for the presence of the auth token in the cookies
    let token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let (uid, _, _) = token
        .ok_or(Error::AuthFailNoAuthToken)
        .and_then(parse_token)?;
    println!("-->> {:<12} - mw_require_auth: {}", "MIDDLEWARE", uid);
    Ok(next.run(req).await)
}

// Parse a token of format "user-[id].[expiration].[signature]"
// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let parts: Vec<&str> = token.split('.').collect();
    println!("-->> {:<12} - parse_token parts: {:?}", "MIDDLEWARE", parts);
    if parts.len() != 3 {
        return Err(Error::AuthFailInvalidToken);
    }
    let parts_id: Vec<&str> = parts[0].split('-').collect();
    if parts_id.len() != 2 {
        return Err(Error::AuthFailInvalidToken);
    }
    let user_id = parts_id[1]
        .parse()
        .map_err(|_| Error::AuthFailInvalidToken)?;
    let expiration = parts[1].to_string();
    let signature = parts[2].to_string();

    Ok((user_id, expiration, signature))
}
