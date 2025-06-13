use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::db::model::UserId;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct TotpCode(String);

impl TotpCode {
  pub fn new() -> Self {
    Self(nanoid::nanoid!(16))
  }
}

#[derive(Debug)]
pub struct TotpSession {
  user_id: UserId,
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
}
