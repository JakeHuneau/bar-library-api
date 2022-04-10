use actix_web::web;
use actix_web::HttpResponse;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IngredientData {
    pub name: String,
    pub quantity: f32,
    pub unit: String,
    pub required: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
}

/// Gets all the ingredients available
#[tracing::instrument(name = "Get all ingredients", skip(pool))]
pub async fn get_all_ingredients(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!("SELECT * FROM ingredients")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(rows) => HttpResponse::Ok().json(
            rows.iter()
                .map(|row| Ingredient {
                    id: row.id,
                    name: row.name.clone(),
                })
                .collect::<Vec<Ingredient>>(),
        ),
        Err(e) => {
            tracing::error!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Tries to add multiple ingredients to the DB
/// TODO: Make async
#[tracing::instrument(name = "Add ingredients to DB", skip(pool, ingredients))]
pub async fn add_ingredients<'a>(
    pool: &PgPool,
    ingredients: &'a Vec<IngredientData>,
) -> Result<HashMap<Uuid, &'a IngredientData>, sqlx::Error> {
    let mut ingredient_id_map = HashMap::new();
    for ingredient in ingredients {
        match add_ingredient_db(pool, ingredient).await {
            Ok(id) => ingredient_id_map.insert(id, ingredient),
            Err(e) => return Err(e),
        };
    }
    Ok(ingredient_id_map)
}

/// Checks the Database if the ingredient already exists. If it does, return its id. If not then
/// it inserts a new entry with the ingredient and returns that id
#[tracing::instrument(
    name = "Checking if new ingredient",
    skip(pool, form),
    fields(
        name = %form.name
    )
)]
pub async fn add_ingredient_db(pool: &PgPool, form: &IngredientData) -> Result<Uuid, sqlx::Error> {
    match sqlx::query!("SELECT id FROM ingredients WHERE name = $1", form.name)
        .fetch_optional(pool)
        .await
    {
        Ok(row) => match row {
            Some(value) => Ok(value.id),
            None => insert_ingredient(pool, form).await,
        },
        Err(e) => {
            tracing::error!("Failed to execute query: {}", e);
            Err(e)
        }
    }
}

/// Inserts a single ingredient to the Database and returns the id
#[tracing::instrument(
    name = "Insert ingredient to DB",
    skip(pool, form),
    fields(
        name = %form.name
    )
)]
pub async fn insert_ingredient(pool: &PgPool, form: &IngredientData) -> Result<Uuid, sqlx::Error> {
    let new_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO ingredients (id, name) VALUES ($1, $2)"#,
        new_id,
        form.name
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {}", e);
        e
    })?;
    Ok(new_id)
}
