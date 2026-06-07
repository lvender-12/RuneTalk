use runetalk::{app::create_app, config::config::load_config};
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")))
        .init();

    let conf = load_config().expect("config error");
    debug!("config : {:?}", conf);

    let host = format!("{}:{}", conf.app.host, conf.app.port);

    let state = runetalk::app::AppState::init(conf).await;
    debug!("State : {:?}", state);

    let app = create_app(state);
    let listener = tokio::net::TcpListener::bind(&host)
        .await
        .expect("Failed to bind TCP Listener");

    info!("Listening on {}", host);
    axum::serve(listener, app).await.expect("Failed to serve");
}
