use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub data: Option<serde_json::Value>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            message: "Success".to_string(),
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self { data, message }
    }
}

impl ErrorResponse {
    pub fn error(message: String) -> Self {
        Self {
            data: None,
            message,
        }
    }

    pub fn error_with_data(data: serde_json::Value, _message: String) -> Self {
        Self {
            data: Some(data),
            message: "Error".to_string(),
        }
    }
}

pub fn success_response<T: Serialize>(data: T) -> (StatusCode, Json<ApiResponse<T>>) {
    (StatusCode::OK, Json(ApiResponse::success(data)))
}

pub fn success_response_with_message<T: Serialize>(
    data: T,
    message: String,
) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::OK,
        Json(ApiResponse::success_with_message(data, message)),
    )
}

pub fn error_response(message: String, status: StatusCode) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse::error(message)))
}
