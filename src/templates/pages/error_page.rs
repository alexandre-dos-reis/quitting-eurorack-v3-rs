use maud::{html, Markup};

use crate::templates::base::page;

pub fn error_page() -> Markup {
    page(html! {
        h1 class="text-center text-4xl mt-5 mb-5" {
            "Oops ! An error occured. Please try again later..."
        }
    })
}
