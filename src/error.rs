use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    LoginFail,

    // Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}

// Never pass server errors to the client
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        // let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        let response =
            (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response();
        // Insert Error into the response
        // response.extensions_mut().insert(self);

        response
    }
}
