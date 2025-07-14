use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub message: Option<String>,
}
