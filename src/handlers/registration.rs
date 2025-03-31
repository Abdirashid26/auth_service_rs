use actix_web::web;
use chrono::NaiveDate;
use sqlx::{PgPool, Pool};
use time::{Date, Month};
use validator::{Validate, ValidationErrors};
use crate::auth::service::AuthService;
use crate::models::dtos::NewUserDto;
use crate::models::user::User;
use crate::utils::global_util::{convert_naivedate_to_timedate, convert_timedate_to_naivedate};
use crate::utils::req_res::UniversalResponse;


// register handler
pub async fn register(
    db_pool : web::Data<PgPool>,
    new_user_dto : web::Json<NewUserDto>,
    auth_service: web::Data<AuthService>
) -> UniversalResponse<Option<User>>{

    // i validate the dto
    match validate_user(&*new_user_dto) {
        Ok(_) => {
            println!("new user dto is valid");
        }
        Err(_) => {
            return UniversalResponse::failed(
                "Invalid new user dto payload".to_string(),
                None,
            )
        }
    }




    // register the new user
    let mut new_user = new_user_dto.into_inner();
    let hashed_password = auth_service.hash_password(&*new_user.password);
    new_user.password = hashed_password.unwrap_or("failed to hash password".to_string());

    let user = User::from(new_user);


    // check the phone number already exists
    let record = sqlx::query!(
    r#"
    SELECT
        EXISTS(SELECT 1 FROM tb_users WHERE email = $1) as "email_exists!",
        EXISTS(SELECT 1 FROM tb_users WHERE phone_number = $2) as "phone_exists!"
    "#,
    user.email,
    user.phone_number,
    )
        .fetch_one(db_pool.get_ref())
        .await;


    if record.is_ok() {

        let record = record.unwrap();

        if record.email_exists {
            return UniversalResponse::failed(
                String::from("User with email exists"),
                None
            )
        }

        if record.phone_exists {
            return UniversalResponse::failed(
                String::from("User with phoneNumber exists"),
                None
            )
        }

    }else{
        return UniversalResponse::failed(
            String::from("Failed to check if user exists"),
            None
        )
    }

    let create_new_user_result = sqlx::query!(
    "INSERT INTO tb_users (
        username, email, password_hash, first_name, last_name, is_active, is_verified,
        last_login, created_at, updated_at, phone_number, date_of_birth,
        profile_picture_url, bio, role
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
    ) RETURNING *",
    user.username,
    user.email,
    user.password_hash,
    user.first_name,
    user.last_name,
    user.is_active,
    user.is_verified,
    user.last_login,
    user.created_at,
    user.updated_at,
    user.phone_number,
    user.date_of_birth,
    user.profile_picture_url,
    user.bio,
    user.role
)
        .fetch_one(db_pool.get_ref()) // assuming you have a database connection pool
        .await;


    match create_new_user_result {
        Ok(row ) => {
            println!("Saved new User");
            UniversalResponse::success(
                String::from("Created new user"),
                Some(
                    User{
                        user_id: row.user_id,
                        username: row.username,
                        email: row.email,
                        password_hash: row.password_hash,
                        first_name: row.first_name,
                        last_name: row.last_name,
                        is_active: row.is_active,
                        is_verified: row.is_verified,
                        last_login: None,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        phone_number: row.phone_number,
                        date_of_birth: row.date_of_birth,
                        profile_picture_url: row.profile_picture_url,
                        bio: row.bio,
                        role: row.role,
                    }
                )
            )
        }

        Err(error) => {
            println!("Error when creating user :{:?}",error);
            UniversalResponse::failed(
                String::from("Failed to create User"),
                None
            )
        }
    }


}

fn validate_user(user: &NewUserDto) -> Result<(), validator::ValidationErrors> {
    user.validate()
}