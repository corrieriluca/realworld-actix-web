use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    configuration::{AppSettings, DatabaseSettings},
    services,
};

/// Get a connection pool to the database specified in the given settings.
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub fn run(listener: TcpListener, configuration: &AppSettings) -> Result<Server, std::io::Error> {
    let db_pool = get_connection_pool(&configuration.database);
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api").configure(services::config))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
