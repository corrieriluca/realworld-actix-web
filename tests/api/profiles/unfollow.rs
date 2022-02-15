use claim::assert_err;
use conduit::domain::auth::create_jwt_for_user;
use serde_json::Value;

use crate::helpers::spawn_app;

async fn unfollow_user(address: &str, username: &str, token: &str) -> reqwest::Response {
    reqwest::Client::new()
        .delete(&format!("{address}/api/profiles/{username}/follow"))
        .header("Authorization", format!("Token {token}"))
        .send()
        .await
        .expect("Failed to execute request.")
}

#[actix_rt::test]
async fn not_authenticated_unfollow_should_return_401() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = reqwest::Client::new()
        .delete(&format!("{}/api/profiles/username/follow", app.address()))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[actix_rt::test]
async fn unknown_user_unfollow_should_return_404() {
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
    let response = unfollow_user(app.address(), "unknown", &token).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[actix_rt::test]
async fn self_unfollow_should_return_422() {
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
    let response = unfollow_user(app.address(), "jack", &token).await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn regular_unfollow_should_return_200() {
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
    .expect("Failed to insert user 1.");

    let jack_token = create_jwt_for_user("jack", app.jwt_secret()).expect("JWT generation failed.");

    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        "john",
        "john@john.com",
        "test1234"
    )
    .execute(app.db_pool())
    .await
    .expect("Failed to insert user 2.");

    sqlx::query!("INSERT INTO followers VALUES ('jack', 'john')")
        .execute(app.db_pool())
        .await
        .expect("Failed to insert in followers.");

    // Act
    let response = unfollow_user(app.address(), "john", &jack_token).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(Value::Bool(false), body["profile"]["following"]);

    assert_err!(
        sqlx::query!(
            r#"
            SELECT *
            FROM followers
            WHERE follower = $1
                AND followed = $2
            "#,
            "jack",
            "john"
        )
        .fetch_one(app.db_pool())
        .await
    );
}

#[actix_rt::test]
async fn unfollow_not_following_should_return_200() {
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
    .expect("Failed to insert user 1.");

    let jack_token = create_jwt_for_user("jack", app.jwt_secret()).expect("JWT generation failed.");

    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        "john",
        "john@john.com",
        "test1234"
    )
    .execute(app.db_pool())
    .await
    .expect("Failed to insert user 2.");

    // Act
    let response = unfollow_user(app.address(), "john", &jack_token).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let body: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(Value::Bool(false), body["profile"]["following"]);
}
