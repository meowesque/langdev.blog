use super::prelude::*;
use crate::{
  auth::AuthService,
  csrf::{CsrfService, CsrfToken},
  db::{Db, model::User},
  email::{self, EmailService},
  totp::{TotpCode, TotpService},
};
use mail_builder::{MessageBuilder, mime::MimePart};
use rocket::{
  http::{Cookie, CookieJar},
  response::{
    Flash, Redirect,
    content::RawHtml,
    status::{Accepted, Unauthorized},
  },
};

#[derive(Debug, FromForm)]
pub struct LoginForm {
  csrf: CsrfToken,
  // TODO(meowesque): Use `email_address::EmailAddress`?
  email: String,
}

#[post("/login", data = "<form>")]
pub async fn post(
  db: &State<Db>,
  csrf: &State<CsrfService>,
  email: &State<EmailService>,
  totp: &State<TotpService>,
  form: Form<LoginForm>,
) -> Result<rocket::response::status::Accepted<Markup>, rocket::response::status::Unauthorized<()>>
{
  if !csrf.validate(&form.csrf).await {
    return Err(rocket::response::status::Unauthorized(()));
  }

  let Some(user) = User::get_by_email(&db, &form.email).await.map_err(|e| {
    ::log::warn!("Failed to retrieve user by email: {:?}", e);
    rocket::response::status::Unauthorized(())
  })?
  else {
    return Err(rocket::response::status::Unauthorized(()));
  };

  let code = totp.create(user.id).await;

  // TODO(meowesque): Remove these needless allocations?
  email
    .send(email::template::totp(user.email.clone(), code.0.clone()))
    .await
    .map_err(|e| {
      ::log::warn!("Failed to send email: {:?}", e);
      rocket::response::status::Unauthorized(())
    })?;

  Ok(rocket::response::status::Accepted(basic::template(html! {
    span { "Nice job!" }
  })))
}

#[get("/login")]
pub async fn get(csrf: &State<CsrfService>) -> Markup {
  let csrf = csrf.issue_token().await;

  basic::template(html! {
    form action="/login" method="POST" class="space-y-4" {
      // NOTE(meowesque): This probably isn't necessary?
      input type="hidden" name="csrf" value=(csrf);

      label
        for="email"
        value="Email Address"
        class="block";

      input
        type="text"
        name="email"
        enctype="multipart/form-data"
        class="border-white border p-4 block";

      button
        type="submit"
        class="bg-radial-[at_25%_25%] from-slate-50 to-slate-300 px-3 py-1.5 hover:cursor" {
        span class="text-slate-950 block" { "Login" }
      }
    }
  })
}

#[get("/login/totp?<code>")]
pub async fn totp(
  cookies: &CookieJar<'_>,
  db: &State<Db>,
  totp: &State<TotpService>,
  auth: &State<AuthService>,
  code: String,
) -> Result<Accepted<Markup>, Flash<Redirect>> {
  let code = TotpCode(code);

  let Some(session) = totp.validate(code).await else {
    return Err(Flash::error(
      Redirect::to("/"),
      "Looks like your code is invalid.",
    ));
  };

  let Some(user) = User::get_by_id(&db, session.user_id).await.map_err(|e| {
    ::log::warn!("Failed to retrieve user by id: {:?}", e);
    Flash::error(Redirect::to("/"), "Unauthorized!")
  })?
  else {
    ::log::warn!("User doesn't exist: {:?}", session.user_id);
    return Err(Flash::error(Redirect::to("/"), "Unauthorized!"));
  };

  let token = auth.create(user.id);

  // TODO(meowesque): Add an expiration
  cookies.add(
    Cookie::build(("TOKEN", token.to_string()))
      .secure(true)
      .http_only(true),
  );

  Ok(rocket::response::status::Accepted(basic::template(html! {
    "You're now logged in " (&user.username)
  })))
}
