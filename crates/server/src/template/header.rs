use super::prelude::*;

pub fn template() -> Markup {
  html! {
      // Header
    header class="py-6 px-6 min-w-screen" {
      // Navigation
      nav class="space-x-8 text-sm flex justify-between items-center" {
        div class="space-x-2 text-slate-50 mb-1 underline underline-offset-2" {
          a href="/recent" { "Recent" }
          a href="/search" { "Search" }
        }
        div class="" {
          a href="/login" class="bg-radial-[at_25%_25%] from-slate-50 to-slate-300 px-3 py-1.5" {
            span class="text-slate-950" { "Login" }
          }
        }
      }
    }
  }
}
