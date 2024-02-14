use crate::{Error, Result};

use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::{async_trait, RequestPartsExt};
use axum::{http::Request, response::Response};
use tower_cookies::Cookies;
// use async_trait::async_trait;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;

// Can use Ctx, Result<Ctx>, Option<Ctx> as the context type
pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("-->> {:<12} - mw_require_auth - {:?}", "MIDDLEWARE", ctx);
    // Call the extractor to get the user_id
    ctx?;
    Ok(next.run(req).await)
}

// Parse a token of format "user-[id].[expiration].[signature]"
// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let parts: Vec<&str> = token.split('.').collect();
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

// Extractor for parsing the user_id from the request headers
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("-->> {:<12} - Ctx", "EXTRACTOR");
        // Extract the cookies from the request
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
        let (uid, _, _) = token
            .ok_or(Error::AuthFailNoAuthToken)
            .and_then(parse_token)?;

        Ok(Ctx::new(uid))
    }
}
