mod models;
mod auth;
mod handlers;
mod db;
mod utils;


use crate::auth::jwt::JwtConfig;
use crate::auth::service::AuthService;
use crate::db::database::connect;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use rand::Rng;
use std::env;
use crate::auth::middleware::{basic_validator, bearer_validator};

// #[actix_web::main]
#[tokio::main]
async fn main()  -> std::io::Result<()> {

    dotenv().ok();

    let jwt_config = JwtConfig{
        secret: env::var("SECRET_KEY").expect("JWT_SECRET must be set"),
        expiration_hours: 1,
    };

    let auth_service = AuthService::new(
        jwt_config,
    );

    let pg_pool = connect().await;



    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(pg_pool.clone()))
            .service(
                web::resource("/api/v1/user/register")
                    .wrap(HttpAuthentication::basic(basic_validator))
                    .route(web::post().to(handlers::registration::register))
            )
            .service(
                web::resource("/api/v1/user/login")
                    .wrap(HttpAuthentication::basic(basic_validator))
                    .route(web::post().to(handlers::auth::login))
            )
            .service(
                web::resource("/api/v1/user/view/{email}")
                    .wrap(HttpAuthentication::bearer(bearer_validator))
                    .route(web::get().to(handlers::view::view_user))
            )
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await

}
