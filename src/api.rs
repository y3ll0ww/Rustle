use rocket::{http::Status, response::status::Custom, serde::json::Json};
use serde::{Deserialize, Serialize};

pub type JsonResponse<T> = Custom<Json<ApiResponse<T>>>;

// Define a custom response type for consistency
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn custom(status: Status, message: String, data: Option<T>) -> JsonResponse<T> {
        Custom(
            status,
            Json(ApiResponse {
                message,
                data,
            })
        )
    }

    pub fn success(message: String, data: Option<T>) -> JsonResponse<T> {
        Self::custom(Status::Ok, message, data)
    }

    pub fn bad_request(message: String) -> JsonResponse<T> {
        Self::custom(Status::BadRequest, message, None)
    }

    pub fn not_found(message: String) -> JsonResponse<T> {
        Self::custom(Status::NotFound, message, None)
    }

    pub fn internal_server_error(message: String) -> JsonResponse<T> {
        Self::custom(Status::InternalServerError, message, None)
    }
}