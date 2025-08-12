use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Meta {
  pub title: String,
  /// Post slug (yes, you get to choose this!)
  pub slug: String,
}
