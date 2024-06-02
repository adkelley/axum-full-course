use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

// Check Cookie existence and validity.
pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|c: Cookie| c.value().to_string());

    // Parse Token
    let (id, exp, sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;

    // TODO: Token validation logic.

    Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`.
/// Returns the (user-id, expiration, signature).
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
