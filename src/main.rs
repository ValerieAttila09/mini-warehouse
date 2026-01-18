mod configs;
mod handlers;
mod models;
mod repositories;
mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.init();

	dotenvy::dotenv().ok();

	let pool = configs::db::init_db_pool().await?;
	let jwt = configs::security::JwtConfig::from_env()?;
	let state = handlers::auth::AppState {pool, jwt };

	let app = routes::build_router(state);

	let raw_host = std::env::var("SERVER_HOST").ok();
	let raw_app_port = std::env::var("SERVER_PORT").ok();
	
	let host = raw_host.as_ref().cloned().unwrap_or_else(|| "0.0.0.0".to_string());
	let port: u16 = raw_app_port.as_ref()
		.and_then(|s| s.parse().ok())
		.unwrap_or(3000);

	tracing::debug!(host=?raw_host, app_port=?raw_app_port, port_env=?raw_app_port, "Resolved server binding env values");
	let addr_str = format!("{}:{}", host, port);
	let listener = tokio::net::TcpListener::bind(&addr_str).await?;
	tracing::info!("Server running on {}", addr_str);
	axum::serve(listener, app).await?;

	Ok(())
}