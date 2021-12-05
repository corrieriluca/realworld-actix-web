use std::net::TcpListener;

use conduit::{
    configuration::{read_configuration, DatabaseSettings},
    startup::get_connection_pool,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/// Spawn a [`TestApp`] with a new random database, bind to a random port on
/// localhost.
pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();

    // Randomize configuration to ensure test isolation
    let configuration = {
        let mut c = read_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = format!("conduit_test_{}", Uuid::new_v4().to_string());
        c
    };

    configure_database(&configuration.database).await;

    let server = conduit::run(listener, &configuration).expect("Failed to run server");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: get_connection_pool(&configuration.database),
    }
}

/// Configure a brand new database for running test on it.
async fn configure_database(config: &DatabaseSettings) {
    // Create the random database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");
}
