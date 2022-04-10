use actix_web::web;
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use sqlx::PgPool;
use uuid::Uuid;

use super::Ingredient;

#[derive(serde::Deserialize)]
pub struct GetKitchenData {
    user_id: Uuid,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UpdateKitchenData {
    pub user_id: Uuid,
    pub ingredients: Vec<Ingredient>,
}

/// Gets everything from a user's kitchen
#[tracing::instrument(
    name = "Get user's kitchen",
    skip(form, pool),
    fields(
        user = %form.user_id
    )
)]
pub async fn get_kitchen(form: Path<GetKitchenData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        SELECT ingredients.id AS id, ingredients.name AS ingredient
        FROM kitchen JOIN ingredients ON kitchen.ingredient_id = ingredients.id
        WHERE user_id = $1
    "#,
        &form.user_id
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(
            rows.iter()
                .map(|row| Ingredient {
                    id: row.id,
                    name: row.ingredient.clone(),
                })
                .collect::<Vec<Ingredient>>(),
        ),
        Err(e) => {
            tracing::error!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Updates a user's kitchen. Does this by deleting everything for that user then
/// starting with a clean slate
#[tracing::instrument(
    name = "Update kitchen",
    skip(form, pool),
    fields(
        user = %form.user_id
    )
)]
pub async fn update_kitchen(
    form: Json<UpdateKitchenData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match delete_user_kitchen(&pool, &form.user_id).await {
        Ok(_) => match populate_user_kitchen(&pool, &form).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Adds everything to a user's kitchen
#[tracing::instrument(
    name = "Populate user kitchen",
    skip(pool, form),
    fields(
        user_id = %form.user_id
    )
)]
pub async fn populate_user_kitchen(
    pool: &PgPool,
    form: &UpdateKitchenData,
) -> Result<(), sqlx::Error> {
    for ingredient in &form.ingredients {
        match insert_user_kitchen(pool, form.user_id, ingredient.id).await {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }
    Ok(())
}

/// Inserts a single row into user kitchen
#[tracing::instrument(name = "Insert into user kitchen", skip(pool))]
pub async fn insert_user_kitchen(
    pool: &PgPool,
    user_id: Uuid,
    ingredient_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO KITCHEN (id, user_id, ingredient_id) VALUES ($1, $2, $3)
    "#,
        Uuid::new_v4(),
        user_id,
        ingredient_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {}", e);
        e
    })?;
    Ok(())
}

/// Attempts to delete everything from a user's kitchen
#[tracing::instrument(name = "Delete user kitchen", skip(pool))]
pub async fn delete_user_kitchen(pool: &PgPool, user_id: &Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"DELETE FROM kitchen WHERE user_id = $1"#, &user_id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {}", e);
            e
        })?;
    Ok(())
}
