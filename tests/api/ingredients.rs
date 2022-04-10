use bar_library_api::routes::Ingredient;

use crate::helpers::spawn_app;

#[tokio::test]
async fn getting_all_ingredients_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/ingredients", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(
        16,
        serde_json::from_str::<Vec<Ingredient>>(response.text().await.expect("abc").as_str())
            .unwrap()
            .len()
    );
}
