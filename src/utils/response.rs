#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub data: T,
  pub message: String,
}

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
  pub fn new(message: String) -> Self {
    Self { data: None, message }
  }
}