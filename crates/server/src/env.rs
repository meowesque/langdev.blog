use std::sync::OnceLock;

pub struct Env {
  pub host: String,
  pub token_secret: String,
  pub smtp_relay: String,
  pub smtp_port: u16,
  pub smtp_user: String,
  pub smtp_pass: String,
  pub postgres_connstr: String,
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
  })
}
