use axum::{Json, extract::Request, response::IntoResponse};
use http::StatusCode;

use crate::common::response::ApiResponse;

pub async fn not_found_middleware(req: Request) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse {
            success: false,
            message: format!("{} Not Found", req.uri().path()),
            data: serde_json::json!({}),
        }),
    )
}
