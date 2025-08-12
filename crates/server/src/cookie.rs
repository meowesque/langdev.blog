use base64::{Engine, prelude::BASE64_STANDARD};

use crate::auth::Token;

#[derive(Debug, thiserror::Error)]
pub enum CookieDeserializationError {
  #[error("Failed to decode cookie value")]
  Base64Decode(#[from] base64::DecodeError),
  #[error("Failed to deserialize cookie value")]
  JsonDeserialize(#[from] serde_json::Error),
}

pub struct TokenCookie(pub Token);

impl std::str::FromStr for TokenCookie {
  type Err = CookieDeserializationError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let json = BASE64_STANDARD.decode(s)?;
    let token = serde_json::from_slice(&json)?;

    Ok(Self(token))
  }
}
