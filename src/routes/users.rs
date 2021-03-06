use actix_web::web;
use actix_web::web::{Form, Json};
use actix_web::HttpResponse;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct NewUserData {
    email: String,
    name: String,
    password: Secret<String>,
}

#[derive(serde::Deserialize)]
pub struct LoginData {
    name: String,
    password: Secret<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UpdatePermissionsData {
    pub referer_id: Uuid,
    pub name: String,
    pub can_write: i16,
    pub can_delete: i16,
    pub can_alter_users: i16,
}

#[derive(serde::Deserialize)]
pub struct UpdatePasswordData {
    name: String,
    password: Secret<String>,
}

#[derive(serde::Deserialize)]
pub struct DeleteUserData {
    name: String,
}

/// Adds a new user
#[tracing::instrument(
    name = "Add user",
    skip(form, pool),
    fields(
        username = %form.name,
        email = %form.email
    )
)]
pub async fn add_user(form: Form<NewUserData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_user(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            sqlx::Error::Database(database_error) => match database_error.code() {
                // Check for duplicate error
                Some(code) => match code.to_string().as_str() {
                    "23505" => {
                        // Duplicate error
                        if database_error.message().contains("users_name_key") {
                            HttpResponse::Conflict().body("username")
                        } else if database_error.message().contains("users_email_key") {
                            HttpResponse::Conflict().body("email")
                        } else {
                            HttpResponse::Conflict().finish()
                        }
                    }
                    _ => HttpResponse::InternalServerError().finish(),
                },
                None => HttpResponse::InternalServerError().finish(),
            },
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

/// Inserts a new user into the database
#[tracing::instrument(name = "Inserting new user", skip(form, pool))]
pub async fn insert_user(pool: &PgPool, form: &NewUserData) -> Result<(), sqlx::Error> {
    let hashed_password =
        hash(&form.password.expose_secret(), DEFAULT_COST).expect("Failed to hash password");
    sqlx::query!(
        r#"
        INSERT INTO users (id, name, password, email, permissions, joined_at)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        Uuid::new_v4(),
        &form.name,
        hashed_password,
        form.email,
        0,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

/// Attempts to sign in a user
#[tracing::instrument(
    name = "sign user in",
    skip(form, pool),
    fields(
        username = %form.name
    )
)]
pub async fn sign_in(form: Form<LoginData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(r#"SELECT password FROM users WHERE name = $1 "#, form.name)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(row) => match row {
            Some(result) => match verify(&form.password.expose_secret(), &result.password) {
                Ok(verified) => match verified {
                    true => HttpResponse::Ok().finish(),
                    false => HttpResponse::Unauthorized().finish(),
                },
                Err(_) => {
                    tracing::error!("Failed to verify password");
                    HttpResponse::InternalServerError().finish()
                }
            },
            None => HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            tracing::error!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// permissions are binary. xyz where x is ability to write, y is ability to delete, z is ability to write
#[tracing::instrument(
    name = "calculate permissions",
    level = "debug",
    fields(
        can_write = %can_write,
        can_delete = %can_delete,
        can_alter_users = %can_alter_users)
)]
pub fn calculate_permission(can_write: i16, can_delete: i16, can_alter_users: i16) -> i16 {
    can_write | (2 * can_delete) | (4 * can_alter_users)
}

/// Attempts to update a user's permissions
#[tracing::instrument(
    name = "update user permissions"
    skip(form, pool),
    fields(
        username = %form.name,
        can_write = %form.can_write,
        can_delete = %form.can_delete,
        can_alter_users = %form.can_alter_users
    )
)]
pub async fn update_permissions(
    form: Json<UpdatePermissionsData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // Check user permissions first
    match user_can_alter_users(&pool, form.referer_id).await {
        Ok(allowed) => match allowed {
            true => (),
            false => return HttpResponse::Forbidden().finish(),
        },
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    match update_permissions_db(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Updates a user's permissions in the database
#[tracing::instrument(name = "Update user permissions in DB", skip(pool, form))]
pub async fn update_permissions_db(
    pool: &PgPool,
    form: &UpdatePermissionsData,
) -> Result<(), sqlx::Error> {
    let new_permission =
        calculate_permission(form.can_write, form.can_delete, form.can_alter_users);
    tracing::debug!("New user permissions: {:?}", new_permission);
    sqlx::query!(
        r#"UPDATE users SET permissions = $1 WHERE name = $2"#,
        new_permission,
        &form.name
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {}", e);
        e
    })?;
    Ok(())
}

/// Checks if a user can make new recipes
#[tracing::instrument(name = "Check if user can write", skip(pool))]
pub async fn user_can_write(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
    match get_user_permission(pool, user_id).await {
        Ok(permissions) => Ok(permissions & 1 == 1),
        Err(e) => Err(e),
    }
}

/// Checks if a user can delete recipes
#[tracing::instrument(name = "Check if user can delete", skip(pool))]
pub async fn user_can_delete(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
    match get_user_permission(pool, user_id).await {
        Ok(permissions) => Ok(permissions >> 1 & 1 == 1),
        Err(e) => Err(e),
    }
}

/// Checks if a user can alter other user's permissions
#[tracing::instrument(name = "Check if user can alter users", skip(pool))]
pub async fn user_can_alter_users(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
    match get_user_permission(pool, user_id).await {
        Ok(permissions) => Ok(permissions >> 2 & 1 == 1),
        Err(e) => Err(e),
    }
}

/// Gets a user's permissions from the Database
#[tracing::instrument(name = "Get user permissions", skip(pool))]
pub async fn get_user_permission(pool: &PgPool, user_id: Uuid) -> Result<i16, sqlx::Error> {
    match sqlx::query!(
        r#"
        SELECT permissions FROM users WHERE id = $1
    "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    {
        Ok(result) => match result {
            None => Ok(0),
            Some(row) => Ok(row.permissions),
        },
        Err(e) => {
            tracing::error!("Failed to execute query: {}", e);
            Err(e)
        }
    }
}

/// Attempts to update a user's password
#[tracing::instrument(
    name = "Update user password",
    skip(form, pool),
    fields(
        username = %form.name
    )
)]
pub async fn update_password(
    form: Form<UpdatePasswordData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match hash(&form.password.expose_secret(), DEFAULT_COST) {
        Ok(new_password) => match update_password_db(&pool, &form, Secret::new(new_password)).await
        {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(e) => {
            tracing::error!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Updates a user's password in the databse
#[tracing::instrument(name = "Update user password in DB", skip(form, pool, new_password))]
pub async fn update_password_db(
    pool: &PgPool,
    form: &UpdatePasswordData,
    new_password: Secret<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET password = $1 WHERE name = $2"#,
        new_password.expose_secret(),
        &form.name
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

/// Deletes a user from the database
#[tracing::instrument(
    name = "Delete user",
    skip(form, pool),
    fields(
        username = %form.name
    )
)]
pub async fn delete_user(form: Form<DeleteUserData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(r#"DELETE FROM users WHERE name = $1"#, &form.name)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
