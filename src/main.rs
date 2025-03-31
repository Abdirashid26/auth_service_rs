mod models;
mod auth;
mod handlers;
mod db;
mod utils;


use std::env;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use hex::encode;
use rand::Rng;
use tracing::callsite::register;
use crate::auth::jwt::JwtConfig;
use crate::auth::middleware::{bearer_validator,basic_validator};
use crate::auth::service::AuthService;
use crate::db::database::connect;

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

        let bearer_auth = HttpAuthentication::bearer(bearer_validator);
        let basic_auth = HttpAuthentication::basic(basic_validator);

        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(pg_pool.clone()))
            .service(
                web::resource("/api/v1/user/register")
                    .wrap(basic_auth)
                    .route(web::post().to(handlers::registration::register))
            )
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await

}
