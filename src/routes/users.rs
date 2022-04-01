use actix_web::web;
use actix_web::web::Form;
use actix_web::HttpResponse;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct NewUserData {
    email: String,
    name: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginData {
    name: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct UpdatePermissionsData {
    name: String,
    can_write: i16,
    can_delete: i16,
    can_alter_users: i16,
}

#[derive(serde::Deserialize)]
pub struct UpdatePasswordData {
    name: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct DeleteUserData {
    name: String,
}

pub async fn add_user(form: Form<NewUserData>, pool: web::Data<PgPool>) -> HttpResponse {
    let hashed_password = hash(&form.password, DEFAULT_COST).expect("Failed to hash password");
    match sqlx::query!(
        r#"
        INSERT INTO users (id, name, password, email, permissions, joined_at)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        Uuid::new_v4(),
        form.name,
        hashed_password,
        form.email,
        0,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn sign_in(form: Form<LoginData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(r#"SELECT password FROM users WHERE name = $1 "#, form.name)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(row) => match row {
            Some(result) => match verify(&form.password, &result.password) {
                Ok(verified) => match verified {
                    true => HttpResponse::Ok().finish(),
                    false => HttpResponse::Unauthorized().finish(),
                },
                Err(_) => {
                    println!("Failed to verify password");
                    HttpResponse::InternalServerError().finish()
                }
            },
            None => HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/* permissions are binary. xyz where x is ability to write, y is ability to delete, z is ability to write */
pub fn calculate_permission(can_write: i16, can_delete: i16, can_alter_users: i16) -> i16 {
    can_write | (2 * can_delete) | (4 * can_alter_users)
}

pub async fn update_permissions(
    form: Form<UpdatePermissionsData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_permission =
        calculate_permission(form.can_write, form.can_delete, form.can_alter_users);
    match sqlx::query!(
        r#"UPDATE users SET permissions = $1 WHERE name = $2"#,
        new_permission,
        &form.name
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_password(
    form: Form<UpdatePasswordData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match hash(&form.password, DEFAULT_COST) {
        Ok(new_password) => {
            match sqlx::query!(
                r#"UPDATE users SET password = $1 WHERE name = $2"#,
                new_password,
                &form.name
            )
            .execute(pool.get_ref())
            .await
            {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => {
                    println!("Failed to execute query: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_user(form: Form<DeleteUserData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(r#"DELETE FROM users WHERE name = $1"#, &form.name)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
