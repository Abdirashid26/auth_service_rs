use chrono::{DateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};
use time::{Date, OffsetDateTime};
use validator::Validate;
use crate::models::user::User;
use crate::utils::global_util::convert_naivedate_to_timedate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NewUserDto {
    #[validate(length(min = 3, message = "Username must be at least 6 characters long"))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,

    #[validate(length(min = 2, max = 50, message = "First name must be between 2 and 50 characters"))]
    pub first_name: Option<String>,

    #[validate(length(min = 2, max = 50, message = "Last name must be between 2 and 50 characters"))]
    pub last_name: Option<String>,

    #[validate(
        length(equal = 12, message = "Phone number must be exactly 12 digits"))]
    pub phone_number: Option<String>,

    pub date_of_birth: Option<NaiveDate>,

    #[validate(url(message = "Invalid URL format for profile picture"))]
    pub profile_picture_url: Option<String>,

    #[validate(length(max = 250, message = "Bio cannot exceed 250 characters"))]
    pub bio: Option<String>,

    #[validate(length(min = 3, message = "Role must be at least 3 characters long"))]
    pub role: Option<String>,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserUpdateDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: Option<bool>,
    pub phone_number: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub phone_number: Option<String>,
    pub date_of_birth: Option<Date>,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
    pub role: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            first_name: Some(user.first_name),
            last_name: Some(user.last_name),
            is_active: user.is_active,
            is_verified: user.is_verified,
            last_login: user.last_login,
            created_at: user.created_at,
            updated_at: user.updated_at,
            phone_number: user.phone_number,
            date_of_birth: Option::from(convert_naivedate_to_timedate(user.date_of_birth.unwrap())),
            profile_picture_url: user.profile_picture_url,
            bio: user.bio,
            role: user.role,
        }
    }
}



impl From<NewUserDto> for User {
    fn from(new_user_dto: NewUserDto) -> Self {
        Self {
            user_id: 0 as i64,
            username: new_user_dto.username,
            email: new_user_dto.email,
            password_hash: new_user_dto.password.to_string(),
            first_name: new_user_dto.first_name.unwrap_or(String::from("not provided")),
            last_name: new_user_dto.last_name.unwrap_or(String::from("not provided")),
            is_active: true,
            is_verified: true,
            last_login: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            phone_number: new_user_dto.phone_number,
            date_of_birth: new_user_dto.date_of_birth,
            profile_picture_url: new_user_dto.profile_picture_url,
            bio: new_user_dto.bio,
            role: new_user_dto.role.unwrap_or("USER".to_string()),
        }
    }
}