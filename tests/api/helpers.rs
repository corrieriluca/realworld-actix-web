use conduit::{
    configuration::{read_configuration, DatabaseSettings},
    startup::get_connection_pool,
    Application,
};
use fake::{Fake, StringFaker};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub(crate) struct TestApp {
    address: String,
    db_pool: PgPool,
    jwt_secret: String,
}

impl TestApp {
    /// Get a reference to the test app's address.
    pub(crate) fn address(&self) -> &str {
        self.address.as_ref()
    }

    /// Get a reference to the test app's DB pool.
    pub(crate) fn db_pool(&self) -> &PgPool {
        &self.db_pool
    }

    /// Get a reference to the test app's jwt secret.
    pub(crate) fn jwt_secret(&self) -> &str {
        self.jwt_secret.as_ref()
    }
}

/// Spawn a [`TestApp`] with a new random database, bind to a random port on
/// localhost, with a random JWT shared secret.
pub(crate) async fn spawn_app() -> TestApp {
    // Randomize configuration to ensure test isolation
    let configuration = {
        let mut c = read_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = format!("conduit_test_{}", Uuid::new_v4().to_string());
        // Use a random OS port
        c.app.port = 0;
        // Generate a random dummy secret for JWT
        const ALPHA_NUM: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        c.app.jwt_secret = StringFaker::with(Vec::from(ALPHA_NUM), 8..12).fake();
        c
    };

    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build the application.");

    let port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: get_connection_pool(&configuration.database),
        jwt_secret: configuration.app.jwt_secret,
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
        .expect("Failed to run migrations on the database");
}
