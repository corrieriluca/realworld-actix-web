use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};

use crate::services;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api").configure(services::config))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
