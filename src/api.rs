use diesel::result::Error as DieselError;
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use serde::{Deserialize, Serialize};

pub type Success<T> = Json<ApiResponse<T>>;
pub type Error<T> = Custom<Json<ApiResponse<T>>>;

pub type Null = String;

// Define a custom response type for consistency
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: String, data: Option<T>) -> Success<T> {
        Json(ApiResponse { message, data })
    }

    pub fn from_error(error: DieselError) -> Error<T> {
        match error {
            DieselError::NotFound => Self::not_found(error.to_string()),
            _ => Self::internal_server_error(error.to_string()),
        }
    }

    pub fn error(status: Status, message: String, data: Option<T>) -> Error<T> {
        Custom(status, Json(ApiResponse { message, data }))
    }

    pub fn bad_request(message: String) -> Error<T> {
        Self::error(Status::BadRequest, message, None)
    }

    pub fn conflict(message: String, data: T) -> Error<T> {
        Self::error(Status::Conflict, message, Some(data))
    }

    pub fn not_found(message: String) -> Error<T> {
        Self::error(Status::NotFound, message, None)
    }

    pub fn internal_server_error(message: String) -> Error<T> {
        Self::error(Status::InternalServerError, message, None)
    }

    pub fn unauthorized(message: String) -> Error<T> {
        Self::error(Status::Unauthorized, message, None)
    }
}
