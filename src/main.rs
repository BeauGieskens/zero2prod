use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{config::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("localhost:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind address");
    run(listener, db_pool)?.await
}
