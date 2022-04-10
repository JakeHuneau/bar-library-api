use actix_web::web;
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use super::{add_ingredients, IngredientData};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RecipeData {
    pub name: String,
    pub ingredients: Vec<IngredientData>,
    pub directions: String,
}

#[derive(serde::Deserialize)]
pub struct DeleteRecipeData {
    name: String,
}

#[derive(serde::Deserialize)]
pub struct GetRecipeData {
    name: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GetRecipesFromIngredientsData {
    pub ingredients: Vec<String>,
    pub wildcard: bool,
}

/// Adds a new recipe to the Database. To do this, it first adds each ingredient to the DB
/// if it does not already exist. Then, it adds the recipe to the DB. Then, it adds the links
/// of all the ingredients and recipe to the DB.
#[tracing::instrument(
    name = "Adding new recipe",
    skip(pool, form),
    fields(
        name = %form.name,
        directions = %form.directions
    )
)]
pub async fn add_recipe(form: Json<RecipeData>, pool: web::Data<PgPool>) -> HttpResponse {
    match add_ingredients(&pool, &form.ingredients).await {
        Ok(ingredient_id_map) => match insert_recipe(&pool, &form).await {
            Ok(recipe_id) => {
                match insert_recipe_ingredients(&pool, &recipe_id, &ingredient_id_map).await {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
            Err(e) => match e {
                sqlx::Error::Database(database_error) => match database_error.code() {
                    // Check for duplicate error
                    Some(code) => match code.to_string().as_str() {
                        "23505" => HttpResponse::Conflict().body("name"), // Duplicate error
                        _ => HttpResponse::InternalServerError().finish(),
                    },
                    None => HttpResponse::InternalServerError().finish(),
                },
                _ => HttpResponse::InternalServerError().finish(),
            },
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Attempts to insert a new recipe to database
#[tracing::instrument(
    name = "Insert recipe to DB",
    skip(pool, form),
    fields(
        name = %form.name,
        directions = %form.directions
    )
)]
pub async fn insert_recipe(pool: &PgPool, form: &RecipeData) -> Result<Uuid, sqlx::Error> {
    let new_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO recipes (id, name, directions) VALUES ($1, $2, $3)"#,
        new_id,
        form.name,
        form.directions
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {}", e);
        e
    })?;
    Ok(new_id)
}

/// Tries to insert all the recipe_ingredients links to the Database
/// TODO: Make async
#[tracing::instrument(name = "Insert into recipe_ingredient", skip(pool, ingredient_id_map))]
pub async fn insert_recipe_ingredients(
    pool: &PgPool,
    recipe_id: &Uuid,
    ingredient_id_map: &HashMap<Uuid, &IngredientData>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    for (ingredient_id, &ingredient) in ingredient_id_map.iter() {
        let new_id = Uuid::new_v4();
        let query_res = sqlx::query!(
            r#"INSERT INTO recipe_ingredients (id, recipe_id, ingredient_id, quantity, unit, required)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
            new_id,
            recipe_id,
            ingredient_id,
            ingredient.quantity,
            ingredient.unit,
            ingredient.required
        ).execute(&mut tx).await;
        if let Err(e) = query_res {
            tracing::error!("Failed to execute query: {}", e);
            return match tx.rollback().await {
                // Cancel everything if there was a problem
                Ok(_) => Err(e),
                Err(rollback_error) => {
                    tracing::error!("Failed to rollback: {}", e);
                    Err(rollback_error)
                }
            };
        }
    }
    tx.commit().await?;
    Ok(())
}

/// Deletes a Recipe from the Database
#[tracing::instrument(
    name = "Delete recipe",
    skip(form, pool),
    fields(
        name = %form.name
    )
)]
pub async fn delete_recipe(form: Path<DeleteRecipeData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(r#"DELETE FROM recipes WHERE name = $1"#, &form.name)
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

/// Tries to get a recipe from the Database given a recipe name.
/// TODO: cache this
#[tracing::instrument(
    name = "Get recipe from name",
    skip(form, pool),
    fields(
        name = %form.name
    )
)]
pub async fn get_recipe(form: Path<GetRecipeData>, pool: web::Data<PgPool>) -> HttpResponse {
    match get_recipe_db(&pool, &form.name)
    .await
    {
        Ok(rows) => match rows {
            None => HttpResponse::NotFound().finish(), // Make sure not empty
            Some(recipe) => HttpResponse::Ok().json(recipe),
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

/// Attempts to get a recipe from the database
#[tracing::instrument(
    name = "Get recipe Databse",
    skip(pool)
)]
pub async fn get_recipe_db(pool: &PgPool, name: &String) -> Result<Option<RecipeData>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            recipes.name AS recipe_name,
            recipes.directions AS directions,
            ingredients.name as ingredient_name,
            quantity,
            unit,
            required
        FROM
            recipes
                JOIN recipe_ingredients ON recipes.id = recipe_ingredients.recipe_id
                JOIN ingredients ON recipe_ingredients.ingredient_id = ingredients.id
        WHERE recipes.name = $1
        "#,
        &name
    )
    .fetch_all(pool)
    .await.map_err(|e| {
        tracing::error!("Failed to execute query: {}", e);
        e
    })?;

    match rows.len() {
            0 => Ok(None),
            _ => Ok(Some(RecipeData {
                name: rows.first().unwrap().recipe_name.clone(),
                directions: rows.first().unwrap().directions.clone(),
                ingredients: rows
                    .iter()
                    .map(|result| IngredientData {
                        name: result.ingredient_name.clone(),
                        quantity: result.quantity.unwrap(),
                        unit: result.unit.clone().unwrap(),
                        required: result.required,
                    })
                    .collect::<Vec<IngredientData>>(),
            })),
        }
}

/// Gets a list of all recipes from the database that include at least all of the given ingredients.
/// A use case is if a user wants to know all drinks that have vodka and lemon juice for ideas of
/// what they might want to buy for their bar.
/// 
/// The logic for checking on wildcard can be seen with recipes R1 = [1, 2, 3], R2 = [1, 2], R3 = [1] and requested ingredients I = [1, 2]
/// If we want to do a wildcard check, then we want any recipe that has at least ingredients 1 and 2.
/// To check this, we filter the recipes and remove any that doesn't have both 1 and 2.
/// This is accomplished with I.all(|ing| recipe.contains(ing))
/// 
/// Then, for the exact match without wildcard, we do the opposite and make sure all the required ingredients are searched for.
#[tracing::instrument(
    name = "Get recipes from ingredients",
    skip(pool, form), 
    fields(
        wildcard = %form.wildcard
    )
)]
pub async fn get_recipes(
    form: Json<GetRecipesFromIngredientsData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    tracing::info!("searching with ingredients {:?}", &form.ingredients);
    match get_potential_recipes(&pool, &form).await {
        Ok(recipes) => {
            tracing::info!("Recipe count: {}", recipes.len());
            match &form.wildcard {
                true => HttpResponse::Ok().json(recipes),
                false => HttpResponse::Ok().json(recipes.into_iter().filter(|recipe| {
                    let current_ingredients = recipe.ingredients.iter().filter(|ingredient| ingredient.required).map(|ingredient| ingredient.name.clone()).collect::<Vec<String>>();
                    tracing::info!("Current recipe: {:?}", recipe.name);
                    tracing::info!("Search ingredients: {:?}", &current_ingredients);
                    tracing::info!("Search ingredients: {:?}", &form.ingredients);
                    current_ingredients.iter().all(|ingredient| form.ingredients.contains(ingredient))
                }).collect::<Vec<RecipeData>>()),
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Gets all potential recipes that include a set of ingredients
#[tracing::instrument("Get potential recipes", skip_all)]
pub async fn get_potential_recipes(
    pool: &PgPool,
    form: &GetRecipesFromIngredientsData,
) -> Result<Vec<RecipeData>, sqlx::Error> {
    tracing::info!(
        "Finding potential recipes for following ingredients: {:?}",
        &form.ingredients
    );
    let rows = sqlx::query!(
        r#"
    SELECT
        DISTINCT(recipes.name) AS name
    FROM 
        recipes
            JOIN recipe_ingredients ON recipes.id = recipe_ingredients.recipe_id
            JOIN ingredients ON recipe_ingredients.ingredient_id = ingredients.id
    WHERE ingredients.name = ANY($1)
    "#, &form.ingredients[..]
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {}", e);
        e
    })?;
    let mut recipes: Vec<RecipeData> = vec![];
    for row in rows {
        match get_recipe_db( pool, &row.name).await {
            Ok(maybe_recipe) => match maybe_recipe {
                None => (),
                Some(recipe) => recipes.push(recipe),
            },
            Err(e) => return Err(e)
        };
    }
    Ok(recipes)
}