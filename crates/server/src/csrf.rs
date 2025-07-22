use chrono::{DateTime, Utc};
use rocket::FromForm;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

const CSRF_TOKEN_LENGTH: usize = 32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, FromForm)]
pub struct CsrfToken(pub String);

impl CsrfToken {
  pub fn new() -> Self {
    Self(nanoid::nanoid!(CSRF_TOKEN_LENGTH))
  }
}

impl std::fmt::Display for CsrfToken {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug)]
pub struct CsrfSession {
  pub issued_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct CsrfService {
  map: Arc<RwLock<HashMap<CsrfToken, CsrfSession>>>,
}

impl CsrfService {
  pub async fn issue_token(&self) -> CsrfToken {
    let token = CsrfToken::new();

    self.map.write().await.insert(
      token.clone(),
      CsrfSession {
        issued_at: Utc::now(),
      },
    );

    token
  }

  pub async fn validate(&self, csrf: &CsrfToken) -> bool {
    self.map.write().await.remove(&csrf).is_some()
  }
}
