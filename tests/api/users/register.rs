use crate::helpers::spawn_app;

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
