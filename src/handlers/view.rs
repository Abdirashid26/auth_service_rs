use actix_web::web;
use sqlx::PgPool;
use crate::auth::service::AuthService;
use crate::models::user::User;
use crate::utils::req_res::UniversalResponse;
use std::time::Instant;


pub async fn view_user(
    db_pool: web::Data<PgPool>,
    _auth_service: web::Data<AuthService>,  // Marked unused with _
    path: web::Path<String>,
) -> UniversalResponse<Option<User>> {
    let email = path.into_inner();

    let start_time = Instant::now();


    // Fetch user from database
    let user_result = sqlx::query_as!(
        User,  // Directly map to User struct
        r#"
        SELECT
            user_id,
            username,
            email,
            password_hash,
            first_name,
            last_name,
            is_active,
            is_verified,
            last_login,
            created_at,
            updated_at,
            phone_number,
            date_of_birth,
            profile_picture_url,
            bio,
            role
        FROM tb_users
        WHERE email = $1
        "#,
        email
    )
        .fetch_optional(db_pool.as_ref())
        .await
        .map_err(|e| {
            log::error!("Database error fetching user: {}", e);
            UniversalResponse::failed(
                "Failed to retrieve user data".to_string(),
                None::<User>  // Explicit type annotation
            )
        });

    let duration = start_time.elapsed();

    println!("Query executed in: {:?}", duration);



    match user_result {
        Ok(user) =>  {
            if user.is_some() {
                UniversalResponse::success(
                    "User details retrieved".to_string(),
                    user
                )
            }
            else{
                UniversalResponse::failed(
                    "User Not Found".to_string(),
                    None::<User>
                )
            }
        },
        Err(err) => UniversalResponse::failed(
            "User not found".to_string(),
            None::<User>  // Explicit type annotation
        ),
    }




}




