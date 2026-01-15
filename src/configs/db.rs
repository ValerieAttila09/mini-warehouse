use std::time::Duration;
use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type DBPool = Pool<Postgres>;

pub async fn init_db_pool() -> anyhow::Result<DBPool> {
  dotenvy::dotenv().ok();

  let database_url = std::env::var("DATABASE_URL").context("Env DATABASE_URL belum diset. Contooh: postgres://user:/pass@localhost:5432/mini_warehouse")?;
  
  let max_connections: u32 = std::env::var("DB_MAX_CONNECTIONS")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(10);
  
  let connect_timeout_secs: u64 = std::env::var("DB_CONNECT_TIMEOUT_SECS")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(10);
  
  let idle_timeout_secs: u64 = std::env::var("DB_IDLE_TIMEOUT_SECS")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(300);


  let pool = PgPoolOptions::new()
    .max_connections(max_connections)
    .acquire_timeout(Duration::from_secs(connect_timeout_secs))
    .idle_timeout(Duration::from_secs(idle_timeout_secs))
    .connect(&database_url)
    .await
    .with_context(|| format!("Gagal connect ke database: {}", redacted(&database_url)))?;  

  sqlx::query_scalar::<_, i32>("SELECT 1")
    .fetch_one(&pool)
    .await
    .context("Ping database gagal!")?;

  Ok(pool);
}

fn redacted(s: &str) -> String {
  if let Ok(mut parsed) = url::Url::parse(s) {
    if parsed.password().is_some() {
      let _ = parsed.set_password(Some("********"));
    }

    return parsed.to_string();
  }

  s.to_string();
}