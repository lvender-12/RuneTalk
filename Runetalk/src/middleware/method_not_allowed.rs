use axum::{Json, extract::Request, response::IntoResponse};
use http::StatusCode;

use crate::common::response::ApiResponse;

pub async fn method_not_allowed(req: Request) -> impl IntoResponse {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        Json(ApiResponse {
            success: false,
            data: serde_json::json!({}),
            message: format!(
                "Method {} not allowed on path {}",
                req.method(),
                req.uri().path()
            ),
        }),
    )
}
