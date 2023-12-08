use maud::{html, Markup};

use crate::{
    api::types::Module,
    templates::{base::page, partials::tag::tag},
};

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
        main class="grid xs:grid-cols-1 sm:grid-cols-2 md:grid-cols-1 gap-5 px-5 mb-24" {
            @for m in &modules {
                article class="border border-gray-600 p-5 rounded-lg" {
                    section class="grid grid-cols-1 md:grid-cols-7 gap-4" {
                        (tag("Manufacturer", &m.manufacturer, "bg-blue-500"))
                        (tag("Name", &m.name, "bg-yellow-500"))
                        (tag("Condition", &m.condition, "bg-pink-500"))
                        (tag("Box included", if m.box_included {"Yes"} else {"No"}, "bg-green-500"))
                        (tag("Price", m.price.to_string().as_str(), "bg-orange-500"))
                        (tag("Rack rash", if m.rack_rash {"Yes"} else {"No"}, "bg-purple-500"))
                    }
                }
            }
        }
    })
}
