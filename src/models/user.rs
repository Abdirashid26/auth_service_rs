use bcrypt::verify;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};



#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_verified: bool,

    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    #[sqlx(try_from = "Option<chrono::DateTime<Utc>>")]
    pub last_login: Option<DateTime<Utc>>,

    #[serde_as(as = "TimestampSeconds<i64>")]
    #[sqlx(try_from = "chrono::DateTime<Utc>")]
    pub created_at: DateTime<Utc>,

    #[serde_as(as = "TimestampSeconds<i64>")]
    #[sqlx(try_from = "chrono::DateTime<Utc>")]
    pub updated_at: DateTime<Utc>,

    pub phone_number: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
    pub role: String,
}

impl User {
    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, &self.password_hash)
    }
}