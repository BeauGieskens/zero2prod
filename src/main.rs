use std::net::TcpListener;

use zero2prod::{
    config::{get_configuration, Settings},
    startup::run,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("localhost:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind address");
    run(listener)?.await
}
