use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;

use zero2prod::configuration;
use zero2prod::startup;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    // Panic if we can't read configuration
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    let connection_pool =
        PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
            // .await
            .expect("Failed to connect to Postgres.");

    startup::run(listener, connection_pool)?.await
}
