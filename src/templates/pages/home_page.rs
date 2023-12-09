use maud::{html, Markup};

use crate::{
    api::types::Module,
    templates::{
        base::page,
        partials::{contact::link, pictures::pictures, tag::tag},
    },
};

pub fn home_page(modules: Vec<Module>, asset_endpoint: &String) -> Markup {
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
                        div class="grid grid-cols-2 justify-center items-center gap-x-5 md:block" {
                            div class="rounded-xl text-end md:text-center text-gray-800 md:mb-3" {
                                span class="bg-red-500 rounded-xl px-2 py-1 whitespace-nowrap" {
                                    "Link"
                                }
                            }
                            div class="rounded-xl text-end md:text-center text-gray-300 md:mb-3" {
                                (link(m))
                            }
                        }
                    }
                    (pictures(m, asset_endpoint))
                }
            }
        }
        div
            id="modal" class="fixed hidden z-50 inset-10"
            _="on click toggle .hidden on me" {
            img
                id="modal-img"
                class="object-contain h-full w-full rounded-lg"
                src="https://office.alexandre-dosreis.me/assets/820d6f3f-0ff8-4fe8-b382-262f96301752"
            ;
        }
    })
}
