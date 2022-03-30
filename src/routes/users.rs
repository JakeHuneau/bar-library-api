use actix_web::web;
use actix_web::web::Form;
use actix_web::HttpResponse;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
    password: String,
}

pub async fn add_user(form: Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
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

// pub async fn sign_in(form: Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
//     match sqlx::query!(r#"SELECT password FROM users WHERE name = $1 "#, form.name)
//         .fetch_optional(pool.get_ref())
//         .await
//     {
//         Ok(row) => match row {
//             Some(result) => match verify(&form.password, &result.password) {
//                 Ok(verified) => match verified {
//                     true => HttpResponse::Ok().finish(),
//                     false => HttpResponse::Unauthorized().finish(),
//                 },
//                 Err(_) => {
//                     println!("Failed to verify password");
//                     HttpResponse::InternalServerError().finish()
//                 }
//             },
//             None => HttpResponse::NotFound().finish(),
//         },
//         Err(e) => {
//             println!("Failed to execute query: {}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

// #[rocket::put("/updatePermissions/<username>")]
// pub fn update_permissions(username: &str) -> String {
//     format!("Updating permissions on {}", username)
// }

// #[rocket::put("/updatePassword/<user_id>")]
// pub fn update_password(user_id: usize) -> String {
//     format!("Updating password on {}", user_id)
// }

// #[rocket::delete("/<username>")]
// pub fn delete_user(username: &str) -> String {
//     format!("Deleting user {}", username)
// }
