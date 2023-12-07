use maud::{html, Markup};

use crate::{api::types::Module, templates::base::page};

pub fn home_page(modules: Vec<Module>) -> Markup {
    page(html! {
        h1 class="text-center text-4xl mt-5 mb-5" {
            "👋 Hello, I'm quitting Eurorack. 😭"
        }
        div class="text-center opacity-50 mb-5" {
            "At least, I'm reducing my rack..."
        }
        h2 class="text-center text-2xl mb-5" {
            "Here's my selling list:"
        }
        @for m in &modules {
         (m.name)
    }
    })
}
