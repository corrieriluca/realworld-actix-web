use std::net::TcpListener;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = conduit::run(listener).expect("Failed to run server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
