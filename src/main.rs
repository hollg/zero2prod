use sqlx::postgres::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application.port);
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        // `connect_with` instead of `connect`
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
