use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpMessage};
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
    let auth_service = req.app_data::<AuthService>().expect("auth service not found in app app data");

    let token = credentials.token();
    
    let validate_token_result = auth_service.validate_token(token);
    
    match validate_token_result {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => {
            Err((actix_web::error::ErrorUnauthorized("Invalid Token"), req))
        }
    }

}



