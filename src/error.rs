use std::alloc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = std::result::Result<T, Error>;

// Server Errors
#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    LoginFail,

    // Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    // Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}

// Never pass server errors to the client
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        // let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        // Fallback might be redundant
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth Errors
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),

            // -- Model Errors
            Self::TicketDeleteFailIdNotFound { id } => {
                (StatusCode::NOT_FOUND, ClientError::INVALID_PARAMS)
            }
            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
// Client Errors
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
