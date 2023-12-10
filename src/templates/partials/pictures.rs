use maud::{html, Markup};

use crate::api::types::Module;

pub fn pictures(m: &Module, asset_endpoint: &String) -> Markup {
    if !m.pictures.is_empty() {
        html! {
            div class="flex flex-wrap justify-center items-center gap-5 mt-9 md:mt-5" {
                @for p in &m.pictures {
                            img
                                class="w-20 h-20 object-cover rounded-md hover:opacity-50 transition-opacity cursor-pointer"
                                src={(asset_endpoint)"/"(p.directus_files_id)"?fit=cover&width=80&height=80&quality=80"}
                                _={"on click put '<img class=\"object-contain h-full w-full rounded-lg\" src=\""(asset_endpoint)"/"(p.directus_files_id)"?fit=cover&width=600&height=600&quality=80\"/>' into #modal then toggle .hidden on #modal"};
                    }
                }
        }
    } else {
        html! {}
    }
}
