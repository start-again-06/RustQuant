use axum::{
    Json
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    Json(payload): Json<LoginRequest>
) -> Json<LoginResponse> {

    println!("Login attempt: {}", payload.username);

    Json(LoginResponse {
        token: "dummy-jwt-token".to_string()
    })
}
