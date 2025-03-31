use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    pub sub : String, // username (in my case the phone number)
    pub exp : usize, // expiration date
    pub role : String // role
}


#[derive(Clone, Debug)]
pub struct JwtConfig{
    pub secret : String,
    pub expiration_hours : i64
}

impl JwtConfig{
    pub fn new(secret: String, expiration_hours : i64) -> JwtConfig{
        Self {
            secret,
            expiration_hours
        }
    }
}