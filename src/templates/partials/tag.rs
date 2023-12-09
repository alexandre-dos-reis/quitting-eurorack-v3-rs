use maud::{html, Markup};

pub fn tag(title: &str, content: &str, color: &str) -> Markup {
    html! {
        div class="grid grid-cols-2 justify-center items-center gap-x-5 md:block" {
            div class="rounded-xl text-end md:text-center text-gray-800 md:mb-3" {
                span class={(color)" rounded-xl px-2 py-1 whitespace-nowrap" } {
                    (title)
                }
            }
            div class="rounded-xl text-end md:text-center text-gray-300 md:mb-3" {
                (content)
            }
        }
    }
}
