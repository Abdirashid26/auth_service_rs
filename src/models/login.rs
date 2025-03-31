use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize,Validate,Debug)]
pub struct LoginDto{
    #[validate(length(min = 3, message = "Username must be at least 6 characters long"))]
    pub username : String,
    #[validate(length(min = 3, message = "Password must be at least 6 characters long"))]
    pub password : String,
}

