use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use std::env;

// Importamos los modelos desde el archivo padre superior
use crate::models::{AuthRequest, User, Claims};

pub async fn register(pool: web::Data<PgPool>, info: web::Json<AuthRequest>) -> impl Responder {
    let hashed_password = match hash(&info.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Error al procesar la contraseña"),
    };

    // pgSQL usa $1, $2 para los bindings
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)"
    )
    .bind(&info.username)
    .bind(hashed_password)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json("Usuario registrado con éxito"),
        Err(_) => HttpResponse::BadRequest().body("El nombre de usuario ya existe o es inválido"),
    }
}

pub async fn login(pool: web::Data<PgPool>, info: web::Json<AuthRequest>) -> impl Responder {
    let user_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&info.username)
        .fetch_optional(pool.get_ref())
        .await;

    if let Ok(Some(user)) = user_result {
        if verify(&info.password, &user.password_hash).unwrap_or(false) {
            
            // ---> AQUÍ FALTA ESTA LÍNEA <---
            // Leemos el secreto del .env o fallamos si no existe
            let secret = env::var("JWT_SECRET").expect("JWT_SECRET debe estar en el .env");

            let expiration = Utc::now() + Duration::days(1);
            let claims = Claims {
                sub: user.id,
                exp: expiration.timestamp(),
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()), // Ahora sí existe 'secret'
            );

            match token {
                Ok(t) => HttpResponse::Ok().json(serde_json::json!({ "token": t })),
                Err(_) => HttpResponse::InternalServerError().body("Error al generar el token"),
            }
        } else {
            HttpResponse::Unauthorized().body("Credenciales incorrectas")
        }
    } else {
        HttpResponse::Unauthorized().body("Credenciales incorrectas")
    }
}