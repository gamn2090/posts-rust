use actix_web::{FromRequest, dev::Payload};
use jsonwebtoken::{decode, Validation, Algorithm, DecodingKey};
use std::future::{ready, Ready};
use std::env;

// estos se llaman extractors porque "extraen" información de la petición HTTP, Fancy name para los middlewares, 
// acá en el de autenticación, extraemos el token JWT del header Authorization y lo validamos para obtener el user_id del usuario autenticado.

use crate::models::{AuthenticatedUser, Claims};

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    
                    // Leemos el secreto desde el .env
                    let secret = env::var("JWT_SECRET").unwrap_or_default();

                    let token_data = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_bytes()),
                        &Validation::new(Algorithm::HS256),
                    );

                    if let Ok(data) = token_data {
                        return ready(Ok(AuthenticatedUser { user_id: data.claims.sub }));
                    }
                }
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized("Token inválido o ausente")))
    }
}