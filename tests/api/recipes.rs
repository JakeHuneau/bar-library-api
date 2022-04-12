use bar_library_api::routes::{
    AddRecipeData, DeleteRecipeData, GetRecipesFromIngredientsData, IngredientData, RecipeData,
};

use crate::helpers::{get_loser_user, get_super_user, spawn_app};

#[tokio::test]
async fn adding_new_recipe_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let recipe = AddRecipeData {
        referer_id: get_super_user(),
        recipe: RecipeData {
            name: String::from("Whiskey and coke"),
            ingredients: vec![
                IngredientData {
                    name: String::from("bourbon"),
                    quantity: 1.0,
                    unit: String::from("shot"),
                    required: true,
                },
                IngredientData {
                    name: String::from("coke"),
                    quantity: 1.0,
                    unit: String::from("cup"),
                    required: true,
                },
            ],
            directions: String::from("mix em"),
        },
    };

    let body = serde_json::to_string(&recipe).expect("Couldn't make JSON");
    let response = client
        .post(&format!("{}/recipe/add_recipe", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let ingredients = sqlx::query!("SELECT COUNT(*) as count FROM ingredients",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch ingredients");

    assert_eq!(Some(17), ingredients.count);

    let recipes = sqlx::query!("SELECT COUNT(*) as count FROM recipes",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch ingredients");

    assert_eq!(Some(5), recipes.count);

    let recipe_ing = sqlx::query!("SELECT COUNT(*) as count FROM recipe_ingredients",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch ingredients");

    assert_eq!(Some(21), recipe_ing.count);
}

#[tokio::test]
async fn adding_new_user_without_permission_gives_403() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let recipe = AddRecipeData {
        referer_id: get_loser_user(),
        recipe: RecipeData {
            name: String::from("Whiskey and coke"),
            ingredients: vec![
                IngredientData {
                    name: String::from("bourbon"),
                    quantity: 1.0,
                    unit: String::from("shot"),
                    required: true,
                },
                IngredientData {
                    name: String::from("coke"),
                    quantity: 1.0,
                    unit: String::from("cup"),
                    required: true,
                },
            ],
            directions: String::from("mix em"),
        },
    };

    let body = serde_json::to_string(&recipe).expect("Couldn't make JSON");
    let response = client
        .post(&format!("{}/recipe/add_recipe", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(403, response.status().as_u16());
}

#[tokio::test]
async fn duplicate_recipe_name_gives_409() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let recipe = AddRecipeData {
        referer_id: get_super_user(),
        recipe: RecipeData {
            name: String::from("old fashioned"),
            ingredients: vec![
                IngredientData {
                    name: String::from("bourbon"),
                    quantity: 1.0,
                    unit: String::from("shot"),
                    required: true,
                },
                IngredientData {
                    name: String::from("coke"),
                    quantity: 1.0,
                    unit: String::from("cup"),
                    required: true,
                },
            ],
            directions: String::from("mix em"),
        },
    };

    let body = serde_json::to_string(&recipe).expect("Couldn't make JSON");
    let response = client
        .post(&format!("{}/recipe/add_recipe", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(409, response.status().as_u16());
}

#[tokio::test]
async fn delete_recipe_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let delete_request = DeleteRecipeData {
        referer_id: get_super_user(),
        name: String::from("godfather"),
    };
    let body = serde_json::to_string(&delete_request).expect("Couldn't make JSON");

    let response = client
        .delete(&format!("{}/recipe/", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let recipes = sqlx::query!("SELECT COUNT(*) as count FROM recipes",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch ingredients");

    assert_eq!(Some(3), recipes.count);

    let recipe_ing = sqlx::query!("SELECT COUNT(*) as count FROM recipe_ingredients",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch ingredients");

    assert_eq!(Some(17), recipe_ing.count);
}

#[tokio::test]
async fn delete_recipe_without_permission_gives_403() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let delete_request = DeleteRecipeData {
        referer_id: get_loser_user(),
        name: String::from("godfather"),
    };
    let body = serde_json::to_string(&delete_request).expect("Couldn't make JSON");

    let response = client
        .delete(&format!("{}/recipe/", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(403, response.status().as_u16());
}

#[tokio::test]
async fn get_recipe_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/recipe/godfather", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let found_recipe =
        serde_json::from_str::<RecipeData>(&response.text().await.expect("Could not get text"))
            .expect("Could not parse json");
    assert_eq!(String::from("godfather"), found_recipe.name);
    assert_eq!(2, found_recipe.ingredients.len());
}

#[tokio::test]
async fn recipe_not_found_returns_404() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/recipe/what", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn get_recipes_exact_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let recipe_search = GetRecipesFromIngredientsData {
        wildcard: false,
        ingredients: vec![
            String::from("scotch"),
            String::from("amaretto"),
            String::from("gin"),
        ],
    };

    let body = serde_json::to_string(&recipe_search).expect("Couldn't make JSON");
    let response = client
        .post(&format!("{}/recipe/", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let found_recipes = serde_json::from_str::<Vec<RecipeData>>(
        &response.text().await.expect("Could not get text"),
    )
    .expect("Could not parse json");
    assert_eq!(1, found_recipes.len());
}

#[tokio::test]
async fn get_recipes_wildcard_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let recipe_search = GetRecipesFromIngredientsData {
        wildcard: true,
        ingredients: vec![
            String::from("scotch"),
            String::from("amaretto"),
            String::from("gin"),
        ],
    };

    let body = serde_json::to_string(&recipe_search).expect("Couldn't make JSON");
    let response = client
        .post(&format!("{}/recipe/", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let found_recipes = serde_json::from_str::<Vec<RecipeData>>(
        &response.text().await.expect("Could not get text"),
    )
    .expect("Could not parse json");
    assert_eq!(3, found_recipes.len());
}
