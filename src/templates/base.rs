use maud::{html, Markup, DOCTYPE};

pub fn page(m: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head
        {
            meta
                charset="utf-8";

            meta
                name="viewport" content="width=device-width, initial-scale=1.0";

            script
                src="https://unpkg.com/htmx.org@1.9.9"
                integrity="sha384-QFjmbokDn2DjBjq+fM+8LUIVrAgqcNW2s0PjAxHETgRn9l4fvX31ZxDxvwQnyMOX"
                crossorigin="anonymous" {}

            title {
                "Quitting Eurorack"
            }
        }
        body
            class="bg-gray-800 text-gray-300 font-mono"
        {
            (m)
        }

        footer
        {
        "Made with Rust, Axum and Maud !"
        }
    }
}
