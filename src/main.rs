use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    color_eyre::install()?;
    setup_tracing();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = axum::Router::new();

    info!("Initializing the Gittr server on port 3000...");
    axum::serve(listener, app).await?;

    Ok(())
}

fn setup_tracing() {
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();

    let fmt_filter = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .pretty();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_filter)
        .init();
}
