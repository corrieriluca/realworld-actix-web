use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, App, HttpServer};

use crate::services::health_check;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(health_check::register)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
