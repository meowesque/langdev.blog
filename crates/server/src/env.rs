use std::{path::PathBuf, sync::OnceLock};

pub struct Env {
  pub host: String,
  pub token_secret: String,
  pub smtp_relay: String,
  pub smtp_port: u16,
  pub smtp_user: String,
  pub smtp_pass: String,
  pub postgres_connstr: String,
  pub content_index_db_path: PathBuf,
  pub content_index_dir: PathBuf,
  pub content_index_temp_dir: PathBuf,
}

pub fn get() -> &'static Env {
  static INSTANCE: OnceLock<Env> = OnceLock::new();

  INSTANCE.get_or_init(|| Env {
    host: std::env::var("HOST").expect("HOST must be present"),
    token_secret: std::env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be present"),
    smtp_relay: std::env::var("SMTP_RELAY").expect("SMTP_RELAY must be present"),
    smtp_port: std::env::var("SMTP_PORT")
      .expect("SMTP_PORT must be present")
      .parse()
      .expect("SMTP_PORT must be a valid u16"),
    smtp_user: std::env::var("SMTP_USER").expect("SMTP_USER must be present"),
    smtp_pass: std::env::var("SMTP_PASS").expect("SMTP_PASS must be present"),
    postgres_connstr: std::env::var("POSTGRES_CONNSTR").expect("POSTGRES_CONNSTR must be present"),
    content_index_db_path: std::env::var("CONTENT_INDEX_DB_PATH")
      .expect("CONTENT_INDEX_DB_PATH must be present")
      .into(),
    content_index_dir: std::env::var("CONTENT_INDEX_DIR")
      .expect("CONTENT_INDEX_DIR must be present")
      .into(),
    content_index_temp_dir: std::env::var("CONTENT_INDEX_TEMP_DIR")
      .expect("CONTENT_INDEX_TEMP_DIR must be present")
      .into(),
  })
}
