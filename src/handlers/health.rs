use axum::{extract::State, response::IntoResponse};
use serde::Serialize;

use crate::handlers::auth::AppState;
use crate::utils::response::success_response;

#[derive(Serialize)]
pub struct HealthResponse {
  pub status: String,
  pub timestamp: String,
  pub database: String,
}

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
  let db_status: String = match sqlx::query_scalar::<_,i32>("SELECT 1").fetch_one(&state.pool).await {
    Ok(_) => "Connected".to_string(),
    Err(_) => "disconnected".to_string(),
  };

  let response = HealthResponse {
    status: "ok".to_string(),
    timestamp: chrono::Utc::now().to_rfc3339(),
    database: db_status,
  };

  success_response(response)
}
