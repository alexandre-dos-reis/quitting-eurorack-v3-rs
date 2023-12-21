use maud::{html, Markup};

use crate::{api::types::Module, config::env_vars::ENV_VARS};

fn get_image_markup(url: String) -> String {
    "<img class=\"object-contain h-full w-full rounded-lg\" src=\"".to_owned()
        + &url
        + "?fit=cover&width=600&height=600&quality=80\"/>"
}

pub fn pictures(m: &Module) -> Markup {
    html! {
        div class="flex flex-wrap justify-center items-center gap-5 mt-9 md:mt-5" {
            @if !m.pictures.is_empty() {
                    @for p in &m.pictures {
                        @let asset_url = ENV_VARS.api_endpoint.to_owned() + "/assets/" + &p.directus_files_id;
                        img
                            class="w-20 h-20 object-cover rounded-md hover:opacity-50 transition-opacity cursor-pointer"
                            src={(asset_url)"?fit=cover&width=81&height=80&quality=80"}
                            _={"on click put '"(get_image_markup(asset_url))"' into #modal then toggle .hidden on #modal"};
                        }
                    } @else {
                "No pictures found."
            }
        }
    }
}
