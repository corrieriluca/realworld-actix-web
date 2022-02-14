use conduit::domain::auth::create_jwt_for_user;
use serde_json::Value;

use crate::helpers::spawn_app;

async fn get_profile(address: &str, username: &str) -> reqwest::Response {
    reqwest::Client::new()
        .get(&format!("{address}/api/profiles/{username}"))
        .send()
        .await
        .expect("Failed to execute request.")
}

#[actix_rt::test]
async fn get_non_unknown_profile_should_return_404() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = get_profile(app.address(), "unknown").await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[actix_rt::test]
async fn get_known_profile_should_return_200() {
    // Arrange
    let app = spawn_app().await;

    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        "jack",
        "jack@jack.com",
        "test1234"
    )
    .execute(app.db_pool())
    .await
    .expect("Failed to insert user.");

    // Act
    let response = get_profile(app.address(), "jack").await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    assert_eq!(Value::String("jack".into()), body["profile"]["username"]);
    assert_eq!(Value::Null, body["profile"]["bio"]);
    assert_eq!(Value::Null, body["profile"]["image"]);
}

#[actix_rt::test]
async fn get_known_profile_authenticated_should_return_200() {
    // Arrange
    let app = spawn_app().await;

    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        "jack",
        "jack@jack.com",
        "test1234"
    )
    .execute(app.db_pool())
    .await
    .expect("Failed to insert user.");

    let token = create_jwt_for_user("jack", app.jwt_secret()).expect("JWT generation failed.");

    // Act
    let response = reqwest::Client::new()
        .get(&format!("{}/api/profiles/jack", app.address()))
        .header("Authorization", format!("Token {token}"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    assert_eq!(Value::String("jack".into()), body["profile"]["username"]);
    assert_eq!(Value::Null, body["profile"]["bio"]);
    assert_eq!(Value::Null, body["profile"]["image"]);
    assert_eq!(Value::Bool(false), body["profile"]["following"])
}
