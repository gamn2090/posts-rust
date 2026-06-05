use serde::{Deserialize, Serialize};

// En Rust TODO es privado papá, así que hay que ponerle pub a todo lo que queremos usar desde otros archivos (como los controladores)

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct PostInput {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}

pub struct AuthenticatedUser {
    pub user_id: i32,
}