use std::str::FromStr;

use super::prelude::*;
use crate::{
  auth::token::{AuthService, Token},
  content::index::{ContentIndex, PostMetadata},
  cookie::TokenCookie,
  csrf::{CsrfService, CsrfToken},
  db::{Db, model::User},
  email::{self, EmailService},
  env,
  totp::{TotpCode, TotpService},
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
  db: &State<Db>,
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

  let user = User::get_by_id(&db, token.payload.user_id)
    .await
    .map_err(|_| BadRequest("User doesnt exist!".into()))?
    .ok_or_else(|| BadRequest("User doesn't exist!".into()))?;

  let archive_path = {
    let mut path = env::get().content_index_temp_dir.clone();
    path.push("temp.zip");
    path
  };

  let output_dir = {
    let mut path = env::get().content_index_dir.clone();
    path.push(nanoid::nanoid!(32));
    path
  };

  form
    .archive
    .persist_to(&archive_path)
    .await
    .map_err(|_| BadRequest("Failed to persist archive file".to_owned()))?;

  let meta = compiler::compile(
    &compiler::options::Options {
      output: std::path::Path::new(&output_dir),
      trim_rootdir: true,
    },
    &archive_path,
  )
  .map_err(|_| BadRequest("Failed to compile archive".to_owned()))?;

  content_index
    .trx(async |t| t.insert_post_metadata(&PostMetadata {
      author_id: user.id,
      author_username: (&user.username).into(),
      slug: (&meta.slug).into(),
      filepath: (&output_dir).into()
    }).await)
    .await
    .map_err(|_| BadRequest("Oops".into()))?;

  let url = format!(
    "{}/~{}/{}",
    env::get().host,
    user.username,
    meta.slug
  );

  Ok(Created::new(url).body("Uploaded successfully!".to_owned()))
}
