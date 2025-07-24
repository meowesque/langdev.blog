use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Meta {
  pub title: String,
  pub slug: String,
}
