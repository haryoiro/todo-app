use actix_web::{http::StatusCode, HttpResponse};
use derive_more::Display;
use serde_derive::Serialize;

#[derive(Debug, Display)]
pub enum ServerError {
    /// a generic error.
    #[display(fmt = "Internal Server Error: {}", _0)]
    GenericError(String),
    /// an io error.
    #[display(fmt = "IO Error: {}", _0)]
    IOError(std::io::Error),
    /// an io error.
    #[display(fmt = "Serde Error: {}", _0)]
    ConversionError(serde_json::Error),
    /// an error from the database.
    #[display(fmt = "Database Error: {}", _0)]
    DatabaseError(diesel::result::Error),
    /// a response error.
    #[display(fmt = "Response Error: {}", _0)]
    ResponseError(Box<dyn actix_web::error::ResponseError>),
    /// an error representing a not found resource.
    #[display(fmt = "Not Found Error: {:?}", _0)]
    NotFoundError(Option<String>),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::GenericError(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                code: 500,
                error: "Internal Server Error".to_string(),
                message: msg.to_string(),
            }),
            ServerError::ResponseError(err) => err.error_response(),
            ServerError::NotFoundError(optional_message) => {
                let message = if let Some(message) = optional_message {
                    message.to_string()
                } else {
                    "Not found".to_string()
                };
                let error_response = ErrorResponse {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    error: "Not Found".to_string(),
                    message,
                };
                HttpResponse::NotFound().json(error_response)
            }
            _ => HttpResponse::build(self.status_code())
                .insert_header(("Content-Type", "application/json"))
                .json(json!({
                    "code": self.status_code().as_u16(),
                    "error": "Internal Server Error".to_string(),
                    "message": "Internal Server Error".to_string(),
                })),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::ResponseError(err) => err.status_code(),
            ServerError::NotFoundError(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(err: serde_json::Error) -> Self {
        ServerError::ConversionError(err)
    }
}

impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> Self {
        ServerError::DatabaseError(err)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::IOError(err)
    }
}

impl From<String> for ServerError {
    fn from(err: String) -> Self {
        ServerError::GenericError(err)
    }
}

impl From<&str> for ServerError {
    fn from(err: &str) -> Self {
        ServerError::GenericError(err.to_string())
    }
}
