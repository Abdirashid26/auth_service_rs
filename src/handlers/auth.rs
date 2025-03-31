use actix_web::web;
use sqlx::{PgPool};
use validator::Validate;
use crate::auth::service::AuthService;
use crate::models::login::LoginDto;
use crate::models::user::User;
use crate::utils::req_res::UniversalResponse;

pub async fn login(
    db_pool: web::Data<PgPool>,
    login_dto: web::Json<LoginDto>,
    auth_service: web::Data<AuthService>,
) -> UniversalResponse<Option<String>> {  // Changed to return token
    // Validate input
    let my_login_dto = login_dto.into_inner();
    my_login_dto.validate().map_err(|_| {
        UniversalResponse::failed(
            "Invalid login credentials".to_string(),
            None::<String>
        )
    }).expect("Invalid login credentials");

    // Find user in database
    let user = sqlx::query!(
        r#"
        SELECT *
        FROM tb_users
        WHERE phone_number = $1 OR email = $1
        "#,
        my_login_dto.username
    )
        .fetch_optional(db_pool.as_ref())
        .await
        .map_err(|e| {
            log::error!("Database error: {}", e);
            return UniversalResponse::failed(
                "Login failed".to_string(),  // Don't reveal specific error to client
                None::<String>
            )
        }).expect("fail to fetch user");

    let user = match user {
        Some(row) => {
            User {
                user_id: row.user_id,
                username: row.username,
                email: row.email,
                password_hash: row.password_hash,
                first_name: row.first_name,
                last_name: row.last_name,
                is_active: row.is_active,
                is_verified: row.is_verified,
                last_login: row.last_login,
                created_at: row.created_at,
                updated_at: row.updated_at,
                phone_number: row.phone_number,
                date_of_birth: row.date_of_birth,
                profile_picture_url: row.profile_picture_url,
                bio: row.bio,
                role: row.role,
            }
        },
        None => return UniversalResponse::failed(
            "Invalid credentials".to_string(),  // Generic message for security
            None::<String>
        ),
    };

    // Verify account status
    if !user.is_active {
        return UniversalResponse::failed(
            "Account is not active".to_string(),
            None::<String>
        );
    }

    // Verify password
    auth_service.validate_password(&my_login_dto.password, &user.password_hash)
        .map_err(|e| {
            log::error!("Password verification error: {}", e);
            return UniversalResponse::failed(
                "Invalid credentials".to_string(),  // Generic message
                None::<String>
            )
        }).expect("password verification error");

    // Generate JWT token
    let token = auth_service.generate_token(&user)
        .map_err(|e| {
            log::error!("Token generation error: {}", e);
            UniversalResponse::failed(
                "Login failed".to_string(),
                None::<String>
            )
        }).expect("token generation error");

    UniversalResponse::success(
        "Login successful".to_string(),
        Some(token)
    )
}



