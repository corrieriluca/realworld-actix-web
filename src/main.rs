use std::net::TcpListener;

use conduit::startup;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    startup::run(listener)?.await?;
    Ok(())
}
