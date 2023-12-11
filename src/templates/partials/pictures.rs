use maud::{html, Markup};

use crate::{api::types::Module, config::env_vars::ENV_VARS};

pub fn pictures(m: &Module) -> Markup {
    html! {
        div class="flex flex-wrap justify-center items-center gap-5 mt-9 md:mt-5" {
    @if !m.pictures.is_empty() {
            @for p in &m.pictures {
                @let api_endpoint = ENV_VARS.api_endpoint.to_owned() + "/assets/" + &p.directus_files_id;
                img
                    class="w-20 h-20 object-cover rounded-md hover:opacity-50 transition-opacity cursor-pointer"
                    src={(api_endpoint)"?fit=cover&width=80&height=80&quality=80"}
                    _={"on click put '<img class=\"object-contain h-full w-full rounded-lg\" src=\""(api_endpoint)"?fit=cover&width=600&height=600&quality=80\"/>' into #modal then toggle .hidden on #modal"};
                }
            } @else {
        "No pictures found."
        }
    }
    }
}
