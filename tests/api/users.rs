use crate::helpers::{get_loser_user, get_super_user, spawn_app};
use bar_library_api::routes::{calculate_permission, UpdatePermissionsData};
use bcrypt::verify;

#[tokio::test]
async fn add_user_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=test2&email=test2%40gmail.com&password=hash";
    let response = client
        .post(&format!("{}/user/add_user", &app.address))
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
async fn add_user_fails_if_user_already_exists() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let saved = sqlx::query!("SELECT name FROM users WHERE name = 'test'",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(saved.name, "test");

    let body = "name=test&email=test2%40test.com&password=hash";
    let response = client
        .post(&format!("{}/user/add_user", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(409, response.status().as_u16());
    assert_eq!(
        "username",
        response.text().await.expect("Could not get text")
    );
}

#[tokio::test]
async fn add_user_fails_if_email_already_exists() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let saved = sqlx::query!("SELECT email FROM users WHERE name = 'test'",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(saved.email, "test@test.com");

    let body = "name=test2&email=test%40test.com&password=hash";
    let response = client
        .post(&format!("{}/user/add_user", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(409, response.status().as_u16());
    assert_eq!("email", response.text().await.expect("Could not get text"));
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
            .post(&format!("{}/user/add_user", app.address))
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

#[tokio::test]
async fn user_can_successfully_log_in() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=test&password=password";
    let response = client
        .post(&format!("{}/user/sign_in", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn user_with_wrong_password_gets_unauthorized_response() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=test&password=badpassword";
    let response = client
        .post(&format!("{}/user/sign_in", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn login_with_nonexistant_username_gets_404() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=doesnotexist&password=password";
    let response = client
        .post(&format!("{}/user/sign_in", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn permissions_calculated_correctly() {
    assert_eq!(0, calculate_permission(0, 0, 0));
    assert_eq!(1, calculate_permission(1, 0, 0));
    assert_eq!(2, calculate_permission(0, 1, 0));
    assert_eq!(3, calculate_permission(1, 1, 0));
    assert_eq!(4, calculate_permission(0, 0, 1));
    assert_eq!(5, calculate_permission(1, 0, 1));
    assert_eq!(6, calculate_permission(0, 1, 1));
    assert_eq!(7, calculate_permission(1, 1, 1));
}

#[tokio::test]
async fn update_permissions_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let new_permissions = UpdatePermissionsData {
        referer_id: get_super_user(),
        name: String::from("test"),
        can_write: 1,
        can_delete: 1,
        can_alter_users: 1,
    };
    let body = serde_json::to_string(&new_permissions).expect("Couldn't make JSON");
    let response = client
        .post(&format!("{}/user/update_permissions", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT permissions FROM users WHERE name = 'test'",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(saved.permissions, 7);
}

#[tokio::test]
async fn update_permissions_without_permission_gives_403() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let new_permissions = UpdatePermissionsData {
        referer_id: get_loser_user(),
        name: String::from("test"),
        can_write: 1,
        can_delete: 1,
        can_alter_users: 1,
    };
    let body = serde_json::to_string(&new_permissions).expect("Couldn't make JSON");
    let response = client
        .post(&format!("{}/user/update_permissions", &app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(403, response.status().as_u16());
}

#[tokio::test]
async fn user_can_change_password() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=test&password=password2";
    let response = client
        .post(&format!("{}/user/update_password", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let body = "name=test&password=password";
    let response = client
        .post(&format!("{}/user/sign_in", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());

    let body = "name=test&password=password2";
    let response = client
        .post(&format!("{}/user/sign_in", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn user_can_be_deleted() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let saved = sqlx::query!("SELECT id FROM kitchen",)
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(6, saved.len());

    let body = "name=test";
    let response = client
        .delete(&format!("{}/user/", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let body = "name=test&password=password";
    let response = client
        .post(&format!("{}/user/sign_in", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(404, response.status().as_u16()); // User shouldn't exist anymore

    let saved = sqlx::query!("SELECT id FROM kitchen",)
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(0, saved.len());
}
