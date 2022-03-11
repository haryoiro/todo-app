use actix_web::{
    http::{
        header::{HeaderValue, CONTENT_TYPE},
        HttpResponse, StatusCode,
    },
    web, Error, HttpRequest, HttpResponse,
};
use serde::Serialize;

#[derive(Debug)]
pub enum ServerError {
    /// a generic error.
    GenericError(String),
    /// an io error.
    IOError(std::io::Error),
    /// an io error.
    ConversionError(serde_json::Error),
    /// an error from the database.
    DatabaseError(diesel::result::Error),
    /// a response error.
    ResponseError(Box<dyn actix_web::error::ResponseError>),
    /// an error representing a not found resource.
    NotFoundError(Option<String>),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::DatabaseError(err) => write!(f, "Database error: {}", err),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::ResponseError(err) => err.error_response(),
            ServerError::NotFoundError(optional_message) => {
                let error_response = ErrorResponse {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    error: "Not Found".to_string(),
                    message: optional_message.unwrap_or_else(|| "Resource not found".to_string()),
                };
                HttpResponse::build(StatusCode::NOT_FOUND)
                    .json(error_response)
                    .finish()
            }
            _ => HttpResponse::build(self.status_code())
                .insert_header(ContentType::JSON)
                .json(json!({
                    "code": self.status_code().as_u16(),
                    "error": "Internal Server Error",
                    "message": self.to_string(),
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

impl From<actix_multipart::MultipartError> for ServerError {
    fn from(err: actix_multipart::MultipartError) -> Self {
        ServerError::GenericError(err.to_string())
    }
}

impl From<actix_web::error::BlockingError<ServerError>> for ServerError {
    fn from(err: actix_web::error::BlockingError<ServerError>) -> Self {
        match err {
            actix_web::error::BlockingError::Error(err) => err,
            actix_web::error::BlockingError::Canceled => {
                ServerError::GenericError("Request canceled".to_string())
            }
        }
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
