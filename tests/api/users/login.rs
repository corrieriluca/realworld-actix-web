use serde_json::Value;

use crate::{helpers::spawn_app, users::register::post_register_with_body};

async fn post_login_with_body(address: &str, body: &'static str) -> reqwest::Response {
    reqwest::Client::new()
        .post(&format!("{}/api/users/login", address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.")
}

#[actix_rt::test]
async fn login_with_no_body_should_return_422() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&format!("{}/api/users/login", app.address()))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn login_with_no_content_type_should_return_422() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&format!("{}/api/users/login", app.address()))
        .body(r#"{"user":{"email":"jake@jake.com","password":"jack"}}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn login_with_incorrect_body_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = post_login_with_body(app.address(), r#"{"user":"invalid_data"}"#).await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn login_with_not_valid_email_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    // Act

    // Invalid email address!
    let response = post_login_with_body(
        app.address(),
        r#"{"user":{"email":"@jake.com","password":"jack"}}"#,
    )
    .await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn login_with_empty_password_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    // Act

    // Invalid email address!
    let response = post_login_with_body(
        app.address(),
        r#"{"user":{"email":"jake@jake.com","password":""}}"#,
    )
    .await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn invalid_user_should_return_403() {
    // Arrange
    let app = spawn_app().await;

    // Act

    // The user has not been inserted!
    let response = post_login_with_body(
        app.address(),
        r#"{"user":{"email":"jake@jake.com","password":"jake1234"}}"#,
    )
    .await;

    // Assert
    assert_eq!(403, response.status().as_u16());
}

#[actix_rt::test]
async fn invalid_password_should_return_403() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    // Act

    // Incorrect password!
    let response = post_login_with_body(
        app.address(),
        r#"{"user":{"email":"jake@jake.com","password":"jack4321"}}"#,
    )
    .await;

    // Assert
    assert_eq!(403, response.status().as_u16());
}

#[actix_rt::test]
async fn valid_login_should_return_200() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    // Act
    let response = post_login_with_body(
        app.address(),
        r#"{"user":{"email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    assert_ne!(Value::Null, body["user"]["username"]);
    assert_ne!(Value::Null, body["user"]["email"]);
    assert_ne!(Value::Null, body["user"]["token"]);

    assert_eq!(Value::Null, body["user"]["bio"]);
    assert_eq!(Value::Null, body["user"]["image"]);
}
