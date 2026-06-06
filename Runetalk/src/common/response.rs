use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    pub data: serde_json::Value,
    pub success: bool,
    pub message: String,
}

impl ApiResponse {
    pub fn success<T: Serialize>(message: &str, data: T) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: serde_json::to_value(data).unwrap_or(serde_json::Value::Null),
        }
    }

    pub fn success_msg(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: serde_json::Value::Null,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: serde_json::Value::Null,
        }
    }
}
