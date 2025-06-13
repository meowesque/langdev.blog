use super::prelude::*;

#[get("/")]
pub fn route() -> Markup {
  basic::template(html! {})
}
