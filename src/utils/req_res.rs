use actix_web::{http, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Debug)]
pub struct UniversalResponse<T>{
    status : String,
    message: String,
    data: T,
}


impl <T>  UniversalResponse<T>{

    pub fn success( message : String, data : T) -> UniversalResponse<T>{
        Self{
            status : "00".to_string(),
            message,
            data,
        }
    }

    pub fn failed( message : String, data : T) -> UniversalResponse<T>{
        Self{
            status : "01".to_string(),
            message,
            data,
        }
    }

}



impl <T : Serialize> Responder for UniversalResponse<T>{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(http::StatusCode::OK)
            .content_type("application/json")
            .json(self)
    }
}