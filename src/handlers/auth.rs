use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::verify;
use serde::{Deserialize, Serialize};

use crate::configs::{
    db::DbPool,
    security::{sign_jwt, JwtConfig},
};
use crate::repositories::user_repository::find_user_with_role_by_email;
use crate::utils::response::{error_response, success_response_with_message};

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

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let found = match find_user_with_role_by_email(&state.pool, &payload.email).await {
        Ok(result) => result,
        Err(_) => {
            return error_response(
                "Database error".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response()
        }
    };

    let (user, role) = match found {
        Some(tuple) => tuple,
        None => {
            return error_response("User not found".to_string(), StatusCode::NOT_FOUND)
                .into_response()
        }
    };

    if !verify(&payload.password, &user.password).unwrap_or(false) {
        return error_response("Invalid credentials".to_string(), StatusCode::UNAUTHORIZED)
            .into_response();
    }

    let role_name = role.unwrap_or_else(|| "keeper".to_string());
    let token = match sign_jwt(&state.jwt, user.id, &user.name, &role_name) {
        Ok(t) => t,
        Err(_) => {
            return error_response(
                "Token generation failed".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response()
        }
    };

    let user_info = UserInfo {
        id: user.id,
        email: user.email,
        role: role_name.to_owned(),
    };

    let response = LoginResponse {
        token,
        user: user_info,
    };

    success_response_with_message(response, "Login successful".to_string()).into_response()
}
