use crate::{Error, Result};

use axum::async_trait;
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::{http::Request, response::Response};
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;

// Can use Ctx, Result<Ctx>, Option<Ctx> as the context type
pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("-->> {:<12} - mw_require_auth - {:?}", "MIDDLEWARE", ctx);
    // Call the extractor to get the user_id
    ctx?;
    Ok(next.run(req).await)
}

// Optimize heavy lifting of Ctx extractor to a separate middleware
pub async fn mw_ctx_resolver(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("-->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");
    let token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    // Compute Result<Ctx>
    let result_ctx = match token
        .ok_or(Error::AuthFailNoAuthToken)
        .and_then(parse_token)
    {
        Ok((uid, _exp, _sign)) => Ok(Ctx::new(uid)),
        Err(e) => Err(e),
    };

    // Remove cookie if something went wrong other than 'AuthFailNoAuthToken'
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthToken)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store the result in the request extensions
    req.extensions_mut().insert(result_ctx);

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
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotFound)?
            .clone()
    }
}
