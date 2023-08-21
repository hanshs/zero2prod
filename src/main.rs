use zero2prod::configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
//     telemetry::init_subscriber(subscriber);
//     // Panic if we can't read configuration
//     let configuration = configuration::get_configuration().expect("Failed to read configuration.");
//     let address = format!(
//         "{}:{}",
//         configuration.application.host, configuration.application.port
//     );
//     let listener = TcpListener::bind(address)?;
//     let connection_pool = PgPoolOptions::new()
//         .acquire_timeout(std::time::Duration::from_secs(2))
//         .connect_lazy_with(configuration.database.with_db());

//     // Build an `EmailClient` using `configuration`
//     let sender_email = configuration
//         .email_client
//         .sender()
//         .expect("Invalid sender email address.");

//     let timeout = configuration.email_client.timeout();
//     let email_client = EmailClient::new(
//         configuration.email_client.base_url,
//         sender_email,
//         configuration.email_client.authorization_token,
//         timeout,
//     );

//     startup::run(listener, connection_pool, email_client)?.await
// }
