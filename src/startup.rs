//! This module holds all the startup logic of the application:
//! - Getting a database connection pool to reuse throughout the app ;
//! - Building a server ready to serve the different services
//!   and binded to the desired address.
//!
//! To summarize, an [`Application`] structure is built from the ground up given
//! a specific configuration (address to bind to, database settings...).

use std::net::TcpListener;

use actix_web::{dev::Server, error, middleware::Logger, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    configuration::{DatabaseSettings, Settings},
    domain::{auth::JwtSecret, error::validation_error},
    handlers, middlewares,
};

/// This structure mainly holds the server ready to serve requests, as well as
/// the port it is listening to (used for tests).
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// Builds the application with the given configuration. Returns the
    /// application ready to be run.
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let db_pool = get_connection_pool(&configuration.database);

        let address = format!("{}:{}", configuration.app.host, configuration.app.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let jwt_secret = JwtSecret(configuration.app.jwt_secret);

        let server = build_server(listener, db_pool, jwt_secret)?;

        Ok(Self { port, server })
    }

    /// Run this application's server until stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    /// Get a reference to the application's port.
    pub fn port(&self) -> u16 {
        self.port
    }
}

/// Get a connection pool to the database specified in the given settings.
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

/// Builds a server ready to serve, listening on the given listener and
/// encapsulating data like a database connection pool and a JWT shared secret.
fn build_server(
    listener: TcpListener,
    db_pool: PgPool,
    jwt_secret: JwtSecret,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let jwt_secret = web::Data::new(jwt_secret);

    // Custom Json extractor configuration
    let json_cfg = web::JsonConfig::default()
        // Only accept application/json content type
        .content_type(|mime| mime == mime::APPLICATION_JSON)
        // Use custom error handler that returns 422 status code and proper
        // error response
        .error_handler(|err, _| {
            let response = validation_error(&format!("{}", err));
            error::InternalError::from_response(err, response).into()
        });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middlewares::AuthenticationMiddlewareFactory)
            .service(web::scope("/api").configure(handlers::config))
            .app_data(json_cfg.clone())
            .app_data(db_pool.clone())
            .app_data(jwt_secret.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
