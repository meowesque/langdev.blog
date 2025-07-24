use std::str::FromStr;

use super::prelude::*;
use crate::{
  auth::token::{AuthService, Token}, content::index::ContentIndex, cookie::TokenCookie, csrf::{CsrfService, CsrfToken}, db::{model::User, Db}, email::{self, EmailService}, totp::{TotpCode, TotpService}
};
use mail_builder::{MessageBuilder, mime::MimePart};
use rocket::{
  data::Capped,
  fs::TempFile,
  http::{Cookie, CookieJar},
  response::{
    Flash, Redirect,
    content::RawHtml,
    status::{Accepted, BadRequest, Created, Unauthorized},
  },
};

#[derive(FromForm)]
struct FormData<'a> {
  // TODO(meowesque): https://docs.rs/rocket/latest/rocket/config/index.html
  archive: TempFile<'a>,
}

#[post("/upload", data = "<form>")]
pub async fn post(
  cookies: &CookieJar<'_>,
  auth: &State<AuthService>,
  content_index: &State<ContentIndex>,
  mut form: Form<FormData<'_>>,
) -> Result<Created<String>, BadRequest<String>> {
  // TODO(meowesque): Validate the archive file.
  // TODO(meowesque): Handle errors better; securely l48 sp.?

  // TODO(meowesque): Create a constant for the cookie name.
  let Some(cookie_token) = cookies.get("TOKEN").map(|x| x.value()) else {
    return Err(BadRequest("Missing authentication token".to_owned()));
  };

  let token = TokenCookie::from_str(cookie_token)
    .map_err(|_| BadRequest("Invalid authentication token".to_owned()))
    .map(|x| x.0)?;

  if !auth.validate(&token) {
    return Err(BadRequest("Invalid authentication token".to_owned()));
  };

  form
    .archive
    .persist_to("./something.zip")
    .await
    .map_err(|_| BadRequest("Failed to persist archive file".to_owned()))?;

  let meta = compiler::compile(
    &compiler::options::Options {
      output: std::path::Path::new("./output"),
      trim_rootdir: true,
    },
    "./something.zip",
  )
  .map_err(|_| BadRequest("Failed to compile archive".to_owned()))?;

  

  Ok(Created::new("some link here").body("Uploaded successfully!".to_owned()))
}
