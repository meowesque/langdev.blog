use mail_builder::{MessageBuilder, mime::MimePart};

use crate::{
  csrf::{CsrfService, CsrfToken},
  db::{Db, model::User},
  email::{self, EmailService},
};

use super::prelude::*;

#[derive(Debug, FromForm)]
struct LoginForm {
  csrf: CsrfToken,
  // TODO(meowesque): Use `email_address::EmailAddress`?
  email: String,
}

#[post("/login", data = "<form>")]
pub async fn post(
  db: &State<Db>,
  csrf: &State<CsrfService>,
  email: &State<EmailService>,
  form: Form<LoginForm>,
) -> Result<
  rocket::response::status::Accepted<String>,
  rocket::response::status::Unauthorized<String>,
> {
  // TODO(meowesque): Validate CSRF

  let Some(user) = User::get_by_email(&db, &form.email)
    .await
    .map_err(|_| // TODO: This is unweildy
    rocket::response::status::Unauthorized("Unauthorized".to_owned()))?
  else {
    return Err(rocket::response::status::Unauthorized(
      "Not found".to_owned(),
    ));
  };

  email.send(email::template::totp()).await.expect("uh");

  Ok(todo!())
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
