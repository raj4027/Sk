use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct SignupData {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

