use std::error::Error;
use std::fmt;

use actix_web::{
    error::{JsonPayloadError, PathError, QueryPayloadError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use serde::Serialize;

pub struct CodestrandsError {
    inner: anyhow::Error,
}

#[derive(Serialize)]
pub struct CodestrandsErrorResponse {
    code: u16,
    error: String,
}

impl fmt::Debug for CodestrandsErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl fmt::Display for CodestrandsErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for CodestrandsErrorResponse {}

impl CodestrandsErrorResponse {
    pub fn new(code: u16, error: String) -> Self {
        Self { code, error }
    }
}

impl CodestrandsError {
    pub fn new(inner: anyhow::Error) -> Self {
        Self { inner }
    }
}

impl fmt::Debug for CodestrandsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Display for CodestrandsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> From<T> for CodestrandsError
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        Self { inner: t.into() }
    }
}

impl ResponseError for CodestrandsError {
    fn status_code(&self) -> StatusCode {
        match self.inner.downcast_ref::<sqlx::Error>() {
            Some(sqlx::Error::RowNotFound) => return StatusCode::NOT_FOUND,
            Some(sqlx::Error::Database(error)) => {
                if let Some(code) = error.code() {
                    if code.to_string().as_str() == "23505" {
                        return StatusCode::CONFLICT;
                    }
                }
            }
            _ => (),
        };

        if let Some(JsonPayloadError::Deserialize(_)) =
            self.inner.downcast_ref::<JsonPayloadError>()
        {
            return StatusCode::BAD_REQUEST;
        }

        if let Some(PathError::Deserialize(_)) = self.inner.downcast_ref::<PathError>() {
            return StatusCode::BAD_REQUEST;
        }

        if let Some(QueryPayloadError::Deserialize(_)) =
            self.inner.downcast_ref::<QueryPayloadError>()
        {
            return StatusCode::BAD_REQUEST;
        }

        if let Some(codestrands_error) = self.inner.downcast_ref::<CodestrandsErrorResponse>() {
            return StatusCode::from_u16(codestrands_error.code).unwrap();
        }

        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = CodestrandsErrorResponse {
            code: status_code.as_u16(),
            error: self.to_string(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

macro_rules! codestrands_error {
    ($code:expr, $error:expr) => {
        return Err($crate::utils::error::CodestrandsError::new(
            $crate::utils::error::CodestrandsErrorResponse::new($code, $error.to_string()).into(),
        ))
    };
}

pub(crate) use codestrands_error;
