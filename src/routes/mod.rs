use axum::{routing::{get, post}, Router};

use crate::handlers::{
  auth::{login, AppState},
  health::health_check,
};

pub fn build_router(state: AppState) -> Router {
  let public_router = Router::new()
    .route("/api/v1/health", get(health_check))
    .route("/api/v1/login", post(login))
    .with_state(state);
  public_router
}