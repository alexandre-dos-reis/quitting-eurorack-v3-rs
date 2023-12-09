use maud::{html, Markup};

use crate::api::types::Module;

pub fn link(m: &Module) -> Markup {
    let class = "underline underline-offset-4 hover:opacity-50";

    match &m.modulargrid {
        Some(link) => {
            if link.starts_with("http") {
                html! {
                    a class=(class) href=(link) target="_blank" rel="noreferrer noopener" {
                        "See offer" span class="relative" { "🎁" }
                    }
                }
            } else {
                html! {
                a
                    class=(class)
                    href={"https://www.modulargrid.net/e/offers/view/"(link)}
                    target="_blank"
                    rel="noreferrer noopener" {
                        "See offer on ModularGrid" span class="relative" {"✨"}
                    }
                }
            }
        }
        None => html! {
            a
            class=(class)
            href={"mailto:ajm.dosreis.daponte@gmail.com?subject="(m.manufacturer)" - "(m.name)} {
            "Email me" span class="relative -top-1" { "📬" }
            }
        },
    }
}

