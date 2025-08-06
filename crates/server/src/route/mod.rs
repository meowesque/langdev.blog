use rocket::Route;

pub mod index;
pub mod invite;
pub mod login;
pub mod post;
pub mod upload;

pub(super) mod prelude;

pub fn routes() -> impl Into<Vec<Route>> {
  rocket::routes![
    index::route,
    login::get,
    login::post,
    login::totp,
    post::get,
    post::get_raw,
    upload::post,
    upload::get,
  ]
}
