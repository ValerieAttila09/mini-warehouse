#[derive(Clone)]
pub struct AppState {
  pub pool: DbPool,
  pub jwt: JwtConfig,
}

#[derive(Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Serialize)]
pub struct UserInfo {
  pub id: i64,
  pub email: String,
  pub role: String,
}

#[derive(Clone)]
pub struct LoginResponse {
  pub token: String,
  pub user: UserInfo,
}
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<(axum::http::StatusCode, Json<crate::utils::response::ApiResponse<LoginResponse>>), (axum::http::StatusCode, Json<crate::utils::response::ApiResponse<LoginResponse>>)> {
  
}