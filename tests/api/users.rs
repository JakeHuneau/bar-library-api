use crate::helpers::spawn_app;
use bcrypt::verify;

#[tokio::test]
async fn add_user_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=test2&email=test2%40gmail.com&password=hash";
    let response = client
        .post(&format!("{}/add_user", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name, password, email FROM users WHERE name = 'test2'",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(saved.name, "test2");
    assert_eq!(
        true,
        verify("hash", &saved.password).expect("Failed to verify")
    );
    assert_eq!(saved.email, "test2@gmail.com");
}

#[tokio::test]
async fn add_user_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("email=test2%40gmail.com&password=hash", "missing the name"),
        ("name=test2&password=hash", "missing the email"),
        ("name=test2&email=test2%40gmail.com", "missing the password"),
        (
            "password=hash",
            "missing the name and email (only password)",
        ),
        (
            "email=test2%40gmail.com",
            "missing the name and password (only email)",
        ),
        ("name=test2", "missing the email and password (only name)"),
        ("", "missing everything"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/add_user", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to excecute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

// #[tokio::test]
// async fn user_can_successfully_log_in() {
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();

//     let body = "name=test&password=password";
//     let response = client
//         .post(&format!("{}/sign_in", app.address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     assert_eq!(200, response.status().as_u16());
// }
