use claim::assert_none;
use serde_json::Value;

use crate::helpers::spawn_app;

pub(crate) async fn post_register_with_body(
    address: &str,
    body: &'static str,
) -> reqwest::Response {
    reqwest::Client::new()
        .post(&format!("{}/api/users", address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.")
}

#[actix_rt::test]
async fn register_with_no_body_should_return_422() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&format!("{}/api/users", app.address()))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn register_with_no_content_type_should_return_422() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&format!("{}/api/users", app.address()))
        .body(r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack"}}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn register_with_incorrect_body_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = post_register_with_body(app.address(), r#"{"user":"invalid_data"}"#).await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn register_with_not_valid_email_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    // Act

    // Invalid email address!
    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"@jake.com","password":"jack"}}"#,
    )
    .await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn register_with_empty_password_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    // Act

    // Invalid email address!
    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":""}}"#,
    )
    .await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn register_with_valid_body_should_return_201() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack"}}"#,
    )
    .await;

    // Assert
    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    assert_ne!(Value::Null, body["user"]["username"]);
    assert_ne!(Value::Null, body["user"]["email"]);
    assert_ne!(Value::Null, body["user"]["token"]);

    assert_eq!(Value::Null, body["user"]["bio"]);
    assert_eq!(Value::Null, body["user"]["image"]);
}

#[actix_rt::test]
async fn register_with_valid_body_persists_in_database() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let _ = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack"}}"#,
    )
    .await;

    // Assert
    let saved = sqlx::query!("SELECT username, email, password, bio, image FROM users")
        .fetch_one(app.db_pool())
        .await
        .expect("Failed to fetch the saved user");

    assert_eq!("jack", saved.username);
    assert_eq!("jake@jake.com", saved.email);
    // SHA3-512 hash of "jack"
    assert_eq!("d309fd6af59c2018f41b3b2285b1570a2ac2fc3d3bbb467f2e74ba5196fa9bde15834ff7eac93de3e8fbf83249d767c0e8d90cdc22fcb6d2785ff91bfbcd79c4", saved.password);
    assert_none!(saved.bio);
    assert_none!(saved.image);
}

#[actix_rt::test]
async fn register_with_already_used_username_or_email_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    // Act & Assert

    // First insertion
    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    // Second insertion, the username is the same
    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"user@domain.com","password":"different"}}"#,
    )
    .await;

    assert_eq!(422, response.status().as_u16());

    // Third insertion, the email address is the same
    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"john","email":"jake@jake.com","password":"john"}}"#,
    )
    .await;

    assert_eq!(422, response.status().as_u16());
}
