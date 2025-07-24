use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Meta {
  pub title: String,
  // TODO(meowesque): Rename to slug
  pub url_safe_title: String,
}
