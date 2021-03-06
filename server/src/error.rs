use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use std::fmt::{Display, Formatter};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    status_code: u16,
    message: String,
    details: Option<String>,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn new(status_code: StatusCode, message: &str, details: Option<String>) -> Self {
        let err = Error {
            status_code: status_code.as_u16(),
            message: message.to_string(),
            details,
        };

        sentry::capture_error(&err);

        err
    }

    pub fn as_http_response(&self) -> HttpResponse {
        if cfg!(debug_assertions) {
            HttpResponse::build(StatusCode::from_u16(self.status_code).unwrap()).json(Error {
                status_code: self.status_code,
                message: self.message.clone(),
                details: self.details.clone(),
            })
        } else {
            HttpResponse::build(StatusCode::from_u16(self.status_code).unwrap()).json(Error {
                status_code: self.status_code,
                message: self.message.clone(),
                details: None,
            })
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for Error {}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        println!("{:#?}", err);

        Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string().as_str(),
            None,
        )
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        println!("{:#?}", err);

        Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error ocurred fetching the resource",
            None,
        )
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        println!("{:#?}", err);

        Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error ocurred fetching the resource",
            None,
        )
    }
}

impl From<sqlx::error::Error> for Error {
    fn from(err: sqlx::error::Error) -> Self {
        println!("{:#?}", err);

        Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error ocurred fetching the resource",
            None,
        )
    }
}
