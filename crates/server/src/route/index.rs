use crate::content::index::ContentIndex;

use super::prelude::*;

#[get("/")]
pub async fn route(index: &State<ContentIndex>) -> Markup {
  let posts = index
    .trx(async |t| t.get_all_post_metadata().await)
    .await
    .unwrap_or_default();

  basic::template(html! {
    div class="flex flex-col" {
      @for post in posts {
        (crate::template::post_card::template(&post))
      }
    }
  })
}
