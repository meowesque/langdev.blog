use super::prelude::*;

pub fn template(markup: Markup) -> Markup {
  html! {
    (DOCTYPE)
    html lang="en" {
      head {
        meta charset="UTF-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        meta http-equiv="X-UA-Compatible" content="ie=edge";

        link rel="stylesheet" href="/styles.css";
        link rel="icon" href="/favicon.svg" sizes="any" type="image/svg+xml";

        title { "langdev.blog" }
      }
      body class="bg-slate-950 text-white min-h-screen flex flex-col" {
        (super::header::template())

        // Main content
        main class="flex-grow" {
          div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8" {
            (markup)
          }
        }

        // Footer
        footer class="text-slate-50" {
          div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-6" {
            div class="flex flex-col md:flex-row justify-end items-center" {
              div class="mb-4 md:mb-0 text-sm text-gray-300 space-x-2 underline underline-offset-2" {
                a { "About" }
                a { "Moderation Log" }
              }
            }
          }
        }

        script {
          r#"
          document.getElementById('mobile-menu-button').addEventListener('click', function() {
            const menu = document.getElementById('mobile-menu');
            menu.classList.toggle('hidden');
          });
          "#
        }
      }
    }
  }
}
