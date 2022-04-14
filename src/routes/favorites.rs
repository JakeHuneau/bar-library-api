use actix_web::web;
use actix_web::web::Json;
use actix_web::HttpResponse;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FavoriteData {
    pub user_id: Uuid,
    pub recipe_id: Uuid,
}

/// Adds a new favorite to the database
#[tracing::instrument(
    name = "Add favorite",
    skip(form, pool),
    fields(
        user_id = %form.user_id,
        recipe_id = %form.recipe_id
    )
)]
pub async fn add_favorite(form: Json<FavoriteData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_favorite(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Attempts to insert favorite to database
#[tracing::instrument(
    name = "Insert favorite",
    skip(form, pool),
    fields(
        user_id = %form.user_id,
        recipe_id = %form.recipe_id
    )
)]
pub async fn insert_favorite(pool: &PgPool, form: &FavoriteData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO favorites (id, user_id, recipe_id) VALUES ($1, $2, $3)"#,
        Uuid::new_v4(),
        form.user_id,
        form.recipe_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {}", e);
        e
    })?;
    Ok(())
}

/// Removes a favorite from the database
#[tracing::instrument(
    name = "Remove favorite",
    skip(form, pool),
    fields(
        user_id = %form.user_id,
        recipe_id = %form.recipe_id
    )
)]
pub async fn remove_favorite(form: Json<FavoriteData>, pool: web::Data<PgPool>) -> HttpResponse {
    match delete_favorite(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Attempts to delete the favorite from the Database
#[tracing::instrument(
    name = "Delete favorite from DB",
    skip(form, pool),
    fields(
        user_id = %form.user_id,
        recipe_id = %form.recipe_id
    )
)]
pub async fn delete_favorite(pool: &PgPool, form: &FavoriteData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"DELETE FROM favorites WHERE user_id = $1 AND recipe_id = $2"#,
        form.user_id,
        form.recipe_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {}", e);
        e
    })?;
    Ok(())
}
