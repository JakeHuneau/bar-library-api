use std::str::FromStr;

use bar_library_api::routes::FavoriteData;
use uuid::Uuid;

use crate::helpers::{get_loser_user, get_super_user, spawn_app};

#[tokio::test]
async fn add_favorite_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let request = FavoriteData {
        user_id: get_super_user(),
        recipe_id: Uuid::from_str("d361ff4e-8b51-4db1-91e1-c4341ffc7e4d")
            .expect("Failed to make UUID"),
    };

    let body = serde_json::to_string(&request).expect("Couldn't make JSON");
    let response = client
        .post(&format!("{}/favorites", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT COUNT(*) as count FROM favorites",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(Some(2), saved.count);
}

#[tokio::test]
async fn remove_favorite_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let request = FavoriteData {
        user_id: get_loser_user(),
        recipe_id: Uuid::from_str("ade3c0a4-8e6e-427a-80d3-8e5466c96eb1")
            .expect("Failed to make UUID"),
    };

    let body = serde_json::to_string(&request).expect("Couldn't make JSON");
    let response = client
        .delete(&format!("{}/favorites", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT COUNT(*) as count FROM favorites",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(Some(0), saved.count);
}
