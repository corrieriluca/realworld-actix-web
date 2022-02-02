use serde_json::Value;

use crate::{helpers::spawn_app, users::register::post_register_with_body};

async fn put_update_with_body(address: &str, body: &'static str, token: &str) -> reqwest::Response {
    reqwest::Client::new()
        .put(&format!("{}/api/user", address))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Token {token}"))
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.")
}

#[actix_rt::test]
async fn update_bio_and_image_should_return_200() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // Act
    let response = put_update_with_body(
        app.address(),
        r#"{"user":{"bio":"This is my bio","image":"https://image.com"}}"#,
        token,
    )
    .await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved =
        sqlx::query!("SELECT username, email, bio, image FROM users WHERE username = 'jack'")
            .fetch_one(app.db_pool())
            .await
            .expect("Failed to fetch user");

    assert_eq!("jack", saved.username);
    assert_eq!("jake@jake.com", saved.email);
    assert_eq!("This is my bio", saved.bio.unwrap());
    assert_eq!("https://image.com", saved.image.unwrap());
}

#[actix_rt::test]
async fn update_everything_should_return_200() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // Act
    let response = put_update_with_body(
        app.address(),
        r#"
        {
            "user": {
                "username": "new_username",
                "email": "username@username.com",
                "password": "new_passord1234",
                "bio": "This is my bio",
                "image": "https://image.com"
            }
        }"#,
        token,
    )
    .await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!(
        "SELECT username, email, password, bio, image FROM users WHERE username = 'new_username'"
    )
    .fetch_one(app.db_pool())
    .await
    .expect("Failed to fetch user");

    assert_eq!("new_username", saved.username);
    assert_eq!("username@username.com", saved.email);
    // SHA3-512 of "new_passord1234"
    assert_eq!("768e355beb99d533c26f794ece10bedc114021ffa3721cb5c87307f1e6f78b2f854b71b08511a0d943e4b7ec49603446d79b7c53b44ed51861aa1f9624f3b29e", saved.password);
    assert_eq!("This is my bio", saved.bio.unwrap());
    assert_eq!("https://image.com", saved.image.unwrap());
}

#[actix_rt::test]
async fn update_nothing_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // Act
    let response = put_update_with_body(app.address(), r#"{"user":{}}"#, token).await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn update_invalid_image_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // Act
    let response =
        put_update_with_body(app.address(), r#"{"user":{"image":"invalid_url"}}"#, token).await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn update_invalid_bio_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // Act

    // More than 140 characters long bio
    let response =
        put_update_with_body(app.address(), r#"{"user":{"bio":"zjlebfoeirghbgcfytuyghbvgcfgxdwrsdfghjkmjlkbhvjgchfxgdwsdfghujlhkbvjchxdftyguhljkbvcxdgdftuygihjlhkbvjchfxdfygiuhijlhkvjchxfgdfyghljbvghjknbvghjnb"}}"#, token).await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn update_invalid_password_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // Act

    // Empty password
    let response = put_update_with_body(app.address(), r#"{"user":{"password":""}}"#, token).await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn update_invalid_username_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // Act

    // Invalid username
    let response = put_update_with_body(
        app.address(),
        r#"{"user":{"username":"a not valid username"}}"#,
        token,
    )
    .await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn update_invalid_email_should_return_422() {
    // Arrange
    let app = spawn_app().await;

    let response = post_register_with_body(
        app.address(),
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack1234"}}"#,
    )
    .await;

    assert_eq!(201, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // Act

    // Invalid email
    let response = put_update_with_body(
        app.address(),
        r#"{"user":{"email":"not_valid_email@"}}"#,
        token,
    )
    .await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}
