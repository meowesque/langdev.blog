mod auth;
mod content;
mod cookie;
mod csrf;
mod db;
mod email;
mod env;
mod error;
mod prelude;
mod route;
mod template;
mod totp;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
  dotenv::dotenv().ok();

  pretty_env_logger::formatted_builder()
    .filter_level(log::LevelFilter::Debug)
    .init();

  tokio_rustls::rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .expect("failed to install aws_lc_rs crypto provider");

  let _ = rocket::build()
    .mount("/", route::routes())
    .mount("/", rocket::fs::FileServer::from("./public").rank(1))
    .mount("/", rocket::fs::FileServer::from("./static").rank(0))
    .manage(csrf::CsrfService::default())
    .manage(auth::AuthService::default())
    .manage(
      email::EmailService::from_env()
        .await
        .expect("failed to start EmailService"),
    )
    .manage(
      db::Db::new(&env::get().postgres_connstr)
        .await
        .expect("failed to start Db"),
    )
    .manage(
      content::index::ContentIndex::new(&env::get().content_index_db_path)
        .await
        .expect("failed to start content index"),
    )
    .manage(totp::TotpService::default())
    .launch()
    .await?;

  Ok(())
}
