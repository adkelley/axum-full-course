use crate::{Error, Result};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use tower_cookies::{Cookie, Cookies};

use crate::web::AUTH_TOKEN;

// Check Cookie existence and validity.
pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|c: Cookie| c.value().to_string());

    // TODO: Implement real auth-token parsing & validation.
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    Ok(next.run(req).await)
}
