use serde_json::Value;

use crate::{helpers::spawn_app, users::register::post_register_with_body};

#[actix_rt::test]
async fn get_user_info_without_token_should_return_401() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/user", app.address()))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[actix_rt::test]
async fn get_user_info_with_bad_header_should_return_401() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/user", app.address()))
        .header("Authorization", "Basic token.goes.here")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[actix_rt::test]
async fn get_user_info_with_invalid_token_should_return_401() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/user", app.address()))
        .header("Authorization", "Token invalid_token")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[actix_rt::test]
async fn get_user_info_after_insertion_should_return_200() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // Act
    let response = client
        .get(&format!("{}/api/user", app.address()))
        .header("Authorization", format!("Token {}", token))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let body: serde_json::Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    assert_ne!(Value::Null, body["user"]["username"]);
    assert_ne!(Value::Null, body["user"]["email"]);
    assert_ne!(Value::Null, body["user"]["token"]);

    assert_eq!(Value::Null, body["user"]["bio"]);
    assert_eq!(Value::Null, body["user"]["image"]);
}
