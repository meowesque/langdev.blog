use std::{collections::HashMap, sync::Arc};
use rocket::{request::{FromParam, FromRequest}, FromForm, FromFormField};
use tokio::sync::RwLock;

use crate::db::model::UserId;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct TotpCode(pub String);

impl TotpCode {
  pub fn new() -> Self {
    Self(nanoid::nanoid!(16))
  }
}

#[derive(Debug)]
pub struct TotpSession {
  pub user_id: UserId,
}

#[derive(Clone, Debug, Default)]
pub struct TotpService {
  map: Arc<RwLock<HashMap<TotpCode, TotpSession>>>,
}

impl TotpService {
  pub async fn create(&self, user_id: UserId) -> TotpCode {
    let code = TotpCode::new();

    let _ = self
      .map
      .write()
      .await
      .insert(code.clone(), TotpSession { user_id });

    code
  }

  pub async fn validate(&self, code: TotpCode) -> Option<TotpSession> {
    self.map.write().await.remove(&code)
  }
}
