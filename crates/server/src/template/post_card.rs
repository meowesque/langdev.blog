use crate::content::index::PostMetadata;

use super::prelude::*;

pub fn template(props: &PostMetadata<'_>) -> Markup {
  html! {
    div class="flex flex-col" {
      span class="text-md font-bold" {
        a href=(format!("/~{}/{}", props.author_username.as_ref(), props.slug.as_ref())) {
          (props.slug.as_ref())
        }
      }
      span class="text-sm" {
        "authored by " a href=(format!("/~{}", props.author_username.as_ref())) class="underline" { (props.author_username.as_ref()) }
      }
    }
  }
}
