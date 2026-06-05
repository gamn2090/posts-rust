use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

// Importamos los modelos desde el archivo padre superior
use crate::models::{Post, PostInput, AuthenticatedUser}; 

pub async fn create_post(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    info: web::Json<PostInput>
) -> impl Responder {
    let result = sqlx::query(
        "INSERT INTO posts (title, body, user_id) VALUES ($1, $2, $3)"
    )
    .bind(&info.title)
    .bind(&info.body)
    .bind(auth.user_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json("Post creado con éxito"),
        Err(_) => HttpResponse::InternalServerError().body("Error al guardar el post"),
    }
}

pub async fn get_all_posts(pool: web::Data<PgPool>) -> impl Responder {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(pool.get_ref())
        .await;

    match posts {
        Ok(lista) => HttpResponse::Ok().json(lista),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener los posts"),
    }
}

pub async fn get_post_by_id(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let post_id = path.into_inner();
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(post_id)
        .fetch_optional(pool.get_ref())
        .await;

    match post {
        Ok(Some(p)) => HttpResponse::Ok().json(p),
        Ok(None) => HttpResponse::NotFound().body("Post no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error en la base de datos"),
    }
}

pub async fn update_post(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<i32>,
    info: web::Json<PostInput>
) -> impl Responder {
    let post_id = path.into_inner();

    let current_post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(post_id)
        .fetch_optional(pool.get_ref())
        .await;

    if let Ok(Some(p)) = current_post {
        if p.user_id != auth.user_id {
            return HttpResponse::Forbidden().body("No tienes permisos para modificar este post");
        }

        let update_result = sqlx::query("UPDATE posts SET title = $1, body = $2 WHERE id = $3")
            .bind(&info.title)
            .bind(&info.body)
            .bind(post_id)
            .execute(pool.get_ref())
            .await;

        match update_result {
            Ok(_) => HttpResponse::Ok().json("Post actualizado con éxito"),
            Err(_) => HttpResponse::InternalServerError().body("Error al actualizar el post"),
        }
    } else {
        HttpResponse::NotFound().body("Post no encontrado")
    }
}

pub async fn delete_post(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
    path: web::Path<i32>
) -> impl Responder {
    let post_id = path.into_inner();

    let current_post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(post_id)
        .fetch_optional(pool.get_ref())
        .await;

    if let Ok(Some(p)) = current_post {
        if p.user_id != auth.user_id {
            return HttpResponse::Forbidden().body("No tienes permisos para eliminar este post");
        }

        let delete_result = sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(post_id)
            .execute(pool.get_ref())
            .await;

        match delete_result {
            Ok(_) => HttpResponse::Ok().json("Post eliminado de forma permanente"),
            Err(_) => HttpResponse::InternalServerError().body("Error al eliminar el post"),
        }
    } else {
        HttpResponse::NotFound().body("Post no encontrado")
    }
}