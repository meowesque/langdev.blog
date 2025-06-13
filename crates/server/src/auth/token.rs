use crate::db::model::UserId;
use crate::env;
use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::{DateTime, Utc, serde::ts_milliseconds};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPayload {
  pub user_id: UserId,
  #[serde(with = "ts_milliseconds")]
  pub issued_at: DateTime<Utc>,
}

impl TokenPayload {
  pub fn sign(&self, secret: impl AsRef<[u8]>) -> TokenSignature {
    let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_ref()).unwrap();

    hmac.update(&self.user_id.0.to_be_bytes());
    hmac.update(&self.issued_at.timestamp().to_be_bytes());

    let signature = hmac.finalize().into_bytes().into();

    TokenSignature(signature)
  }

  pub fn sign_from_env(&self) -> TokenSignature {
    self.sign(env::get().token_secret.as_bytes())
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct TokenSignature([u8; 32]);

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
  pub payload: TokenPayload,
  pub signature: TokenSignature,
}

impl Token {
  pub fn new(payload: TokenPayload, secret: impl AsRef<[u8]>) -> Self {
    Self {
      signature: payload.sign(secret),
      payload,
    }
  }

  pub fn new_from_env(payload: TokenPayload) -> Self {
    Self::new(payload, env::get().token_secret.as_bytes())
  }
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      BASE64_STANDARD.encode(serde_json::to_string(self).expect("to encode token as JSON"))
    )
  }
}

#[derive(Clone)]
pub struct AuthService;

impl AuthService {
  pub fn new() -> Self {
    Self
  }

  pub fn create(&self, user_id: UserId) -> Token {
    Token::new_from_env(TokenPayload {
      user_id,
      issued_at: Utc::now(),
    })
  }

  pub fn validate(&self, token: &Token) -> bool {
    let signature = token.payload.sign_from_env();
    token.signature == signature
  }
}

impl Default for AuthService {
  fn default() -> Self {
    Self::new()
  }
}
