use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::controllers::auth::{login, register};
use crate::controllers::posts::{create_post, delete_post, get_all_posts, get_post_by_id, update_post};

// declaramos que existen otros archivos/módulos en nuestro proyecto
// basicamente es mapear los otros archivos para poder usarlos desde aquí, 
// el main es el punto de entrada, así que desde acá se importan los demás 
// módulos y se arma la aplicación.
mod models;
mod extractors;
mod controllers;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Tomamos la URL de la base de datos de las variables de entorno de Docker
    let db_url = env::var("DATABASE_URL").expect("La variable DATABASE_URL debe estar configurada");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("No se pudo conectar a PostgreSQL");

    // Creación de tablas adaptadas a PostgreSQL (SERIAL en lugar de AUTOINCREMENT)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR NOT NULL UNIQUE,
            password_hash VARCHAR NOT NULL
        );"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts (
            id SERIAL PRIMARY KEY,
            title VARCHAR NOT NULL,
            body TEXT NOT NULL,
            user_id INTEGER NOT NULL REFERENCES users(id)
        );"
    ).execute(&pool).await.unwrap();

    println!("Servidor HTTP iniciado correctamente en el puerto 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/auth/register", web::post().to(register))
            .route("/auth/login", web::post().to(login))
            .route("/posts", web::post().to(create_post))
            .route("/posts", web::get().to(get_all_posts))
            .route("/posts/{id}", web::get().to(get_post_by_id))
            .route("/posts/{id}", web::put().to(update_post))
            .route("/posts/{id}", web::delete().to(delete_post))
    })
    // 0.0.0.0 es necesario en Docker para exponer el puerto hacia afuera del contenedor
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}