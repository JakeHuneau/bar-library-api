use std::str::FromStr;

use bar_library_api::routes::{Ingredient, UpdateKitchenData};
use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn getting_user_kitchen_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let user_id =
        Uuid::from_str("be1d71f9-cdcd-4ecd-8fab-e799629c14e2").expect("Couldn't make UUID");
    let response = client
        .get(&format!("{}/kitchen/{}", &app.address, user_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(
        6,
        serde_json::from_str::<Vec<Ingredient>>(response.text().await.expect("abc").as_str())
            .unwrap()
            .len()
    )
}

#[tokio::test]
async fn updating_kitchen_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let saved = sqlx::query!(
        "SELECT COUNT(*) as count FROM kitchen WHERE user_id = 'be1d71f9-cdcd-4ecd-8fab-e799629c14e2'",
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved user");

    assert_eq!(Some(6), saved.count);

    let ingredients = UpdateKitchenData {
        user_id: Uuid::from_str("be1d71f9-cdcd-4ecd-8fab-e799629c14e2")
            .expect("Couldn't make UUID"),
        ingredients: vec![Ingredient {
            id: Uuid::from_str("b64e1c24-c196-4b62-b7dd-61f49b99d757").expect("Couldn'tmake UUid"),
            name: String::from("Ingredient"),
        }],
    };

    let body = serde_json::to_string(&ingredients).expect("Couldn't make JSON");
    let response = client
        .post(&format!("{}/kitchen/", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!(
        "SELECT COUNT(*) as count FROM kitchen WHERE user_id = 'be1d71f9-cdcd-4ecd-8fab-e799629c14e2'",
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved user");

    assert_eq!(Some(1), saved.count);
}
