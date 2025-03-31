use bcrypt::{verify, DEFAULT_COST};
use chrono::{Utc, Duration};
use crate::auth::jwt::{Claims, JwtConfig};
use crate::models::user::User;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};



#[derive(Debug,Clone)]
pub struct AuthService {
    jwt_config : JwtConfig,
}


impl AuthService {

    pub fn new(jwt_config : JwtConfig) -> AuthService {
        Self { jwt_config }
    }

    pub fn get_jwt_config(&self) -> JwtConfig {
        self.jwt_config.clone()
    }

    pub fn validate_password(&self, password : &str, hashed_password : &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password,hashed_password)
    }

    pub fn hash_password(&self, password : &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, DEFAULT_COST)
    }

    pub fn generate_token(&self, user : &User) -> Result<String, jsonwebtoken::errors::Error> {

        let expiration_date = Utc::now()
            .checked_add_signed(Duration::hours(self.jwt_config.expiration_hours))
            .expect("Invalid timestamp")
            .timestamp();

        let claims = Claims{
            sub: user.username.clone(),
            exp: expiration_date as usize,
            role: user.role.clone(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_ref()),
        )

    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_config.secret.as_ref()),
            &Validation::default(),
        ).map(|data| data.claims)
    }

}