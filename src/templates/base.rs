use maud::{html, Markup, DOCTYPE};

pub fn page(m: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link href="/assets/styles.css" rel="stylesheet";
            script
                src="https://unpkg.com/htmx.org@1.9.9"
                integrity="sha384-QFjmbokDn2DjBjq+fM+8LUIVrAgqcNW2s0PjAxHETgRn9l4fvX31ZxDxvwQnyMOX"
                crossorigin="anonymous" {}
            script
                src="https://unpkg.com/hyperscript.org@0.9.12" {}
            title {
                "Quitting Eurorack"
            }
        }

        body class="bg-gray-800 text-gray-300 font-mono" {
            (m)
        }

        footer class="text-center m-4" {
            "Made with Rust, Axum and Maud !"
        }
    }
}
