use tokio_rustls::rustls::crypto::aws_lc_rs;

mod auth;
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
    .expect("to install aws_lc_rs crypto provider");

  let _rocket = rocket::build()
    .mount(
      "/",
      rocket::routes![route::index::route, route::login::get, route::login::post],
    )
    .mount("/", rocket::fs::FileServer::from("./public").rank(1))
    .mount("/", rocket::fs::FileServer::from("./static").rank(0))
    .manage(csrf::CsrfService::default())
    .manage(
      email::EmailService::from_env()
        .await
        .expect("to start EmailService"),
    )
    .launch()
    .await?;

  Ok(())
}
