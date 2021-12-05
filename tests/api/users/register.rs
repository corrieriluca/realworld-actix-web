use crate::helpers::spawn_app;

pub async fn post_register_with_body(address: &str, body: &'static str) -> reqwest::Response {
    reqwest::Client::new()
        .post(&format!("{}/api/users", address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.")
}

#[actix_rt::test]
async fn register_with_no_body_should_return_400() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&format!("{}/api/users", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[actix_rt::test]
async fn register_with_no_content_type_should_return_400() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&format!("{}/api/users", address))
        .body(r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack"}}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[actix_rt::test]
async fn register_with_incorrect_body_should_return_400() {
    // Arrange
    let address = spawn_app();

    // Act
    let response = post_register_with_body(&address, r#"{"user":"invalid_data"}"#).await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[actix_rt::test]
async fn register_with_not_valid_body_should_return_400() {
    // Arrange
    let address = spawn_app();

    // Act

    // Invalid email address!
    let response = post_register_with_body(
        &address,
        r#"{"user":{"username":"jack","email":"@jake.com","password":"jack"}}"#,
    )
    .await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[actix_rt::test]
async fn register_with_valid_body_should_return_200() {
    // Arrange
    let address = spawn_app();

    // Act
    let response = post_register_with_body(
        &address,
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack"}}"#,
    )
    .await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

/*
#[actix_rt::test]
async fn register_with_valid_body_persists_in_database() {
    // Arrange
    let address = spawn_app();

    // Act
    let response = post_register_with_body(
        &address,
        r#"{"user":{"username":"jack","email":"jake@jake.com","password":"jack"}}"#,
    )
    .await;

    // Assert
    todo!()
}
*/
