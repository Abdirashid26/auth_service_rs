use std::sync::Arc;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{web, Error, HttpMessage};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::auth::jwt::Claims;
use crate::auth::service::AuthService;


const USERNAME: &str = "my_name";
const PASSWORD: &str = "my_password";

pub async fn basic_validator(
    req : ServiceRequest,
    credentials : BasicAuth,
) ->  Result<ServiceRequest , (Error,ServiceRequest)>{

    let username = credentials.user_id();
    let password = credentials.password().unwrap_or("");

    if username == USERNAME || password == PASSWORD {
        Ok(req)
    }else {
        Err((actix_web::error::ErrorUnauthorized("Invalid basic auth credentials"), req))
    }

}


pub async fn bearer_validator(
    req : ServiceRequest,
    credentials : BearerAuth,
) -> Result<ServiceRequest, (Error,ServiceRequest)>{
    let auth_service = req.app_data::<web::Data<AuthService>>()
        .expect("AuthService not found in app data");

    match auth_service.validate_token(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        },
        Err(e) => {
            log::warn!("Token validation failed: {}", e);
            Err((
                actix_web::error::ErrorUnauthorized("Invalid Token"),
                req
            ))
        }
    }

}



