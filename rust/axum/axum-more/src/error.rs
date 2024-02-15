use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "status", content = "data")]
pub enum Error {
    LoginFail,
    AuthFailCtxNotFound,
    AuthFailNoAuthToken,
    AuthFailInvalidToken,
    TicketIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("-->> {:<12} - {self:?}", "INTO_RES");
        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        // Insert the error message into the response
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Error::LoginFail => (StatusCode::FORBIDDEN, ClientError::LoginFail),
            Error::AuthFailCtxNotFound
            | Error::AuthFailNoAuthToken
            | Error::AuthFailInvalidToken => (StatusCode::FORBIDDEN, ClientError::NoAuth),
            Error::TicketIdNotFound { id: _ } => {
                (StatusCode::BAD_REQUEST, ClientError::InvalidParams)
            }
        }
    }
}

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum ClientError {
    LoginFail,
    NoAuth,
    InvalidParams,
    ServiceError,
}
