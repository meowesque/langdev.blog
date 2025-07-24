pub(super) use crate::template::basic;
pub(super) use maud::{Markup, html};
use rocket::request::FromParam;
pub(super) use rocket::{form::Form, *};

pub struct AuthorSlug<'a>(pub &'a str);

impl<'a> FromParam<'a> for AuthorSlug<'a> {
  type Error = &'a str;

  fn from_param(param: &'a str) -> Result<Self, Self::Error> {
    if param.starts_with('~') {
      Ok(AuthorSlug(&param[1..]))
    } else {
      Err("Author slug must start with '~'")
    }
  }
}
