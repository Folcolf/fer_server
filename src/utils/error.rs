use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InternalServerError,
    InvalidToken,
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    AdminRequired,
    NotValid,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::NotValid | Self::InvalidToken | Self::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Bad request")
            }
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            Self::InternalServerError | Self::TokenCreation => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::AdminRequired => (StatusCode::FORBIDDEN, "Forbidden"),
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
