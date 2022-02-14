use claim::assert_ok;
use conduit::domain::auth::create_jwt_for_user;

use crate::helpers::spawn_app;

async fn follow_user(address: &str, username: &str, token: &str) -> reqwest::Response {
    reqwest::Client::new()
        .post(&format!("{address}/api/profiles/{username}/follow"))
        .header("Authorization", format!("Token {token}"))
        .send()
        .await
        .expect("Failed to execute request.")
}

#[actix_rt::test]
async fn not_authenticated_follow_should_return_401() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = reqwest::Client::new()
        .post(&format!("{}/api/profiles/username/follow", app.address()))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[actix_rt::test]
async fn unknown_user_follow_should_return_404() {
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
    let response = follow_user(app.address(), "unknown", &token).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[actix_rt::test]
async fn self_follow_should_return_422() {
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
    let response = follow_user(app.address(), "jack", &token).await;

    // Assert
    assert_eq!(422, response.status().as_u16());
}

#[actix_rt::test]
async fn regular_follow_should_return_200() {
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
    let response = follow_user(app.address(), "john", &jack_token).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    assert_ok!(
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
async fn already_follow_should_return_422() {
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
    let response1 = follow_user(app.address(), "john", &jack_token).await;
    let response2 = follow_user(app.address(), "john", &jack_token).await;

    // Assert
    assert_eq!(200, response1.status().as_u16());
    assert_eq!(422, response2.status().as_u16());

    assert_ok!(
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
