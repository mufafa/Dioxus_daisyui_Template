#[allow(non_snake_case)]
use dioxus::prelude::*;

use document::Link;
mod preview;


// paths to assets
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// entry point
fn main() {
    dioxus::launch(App);
}

// main app component
#[component]
fn App() -> Element {
    let mut theme_data: Signal<String> = use_signal(|| "aqua".to_string());
    rsx! {
        Link { rel: "icon", href: FAVICON } // link our favicon
        Link { rel: "stylesheet", href: MAIN_CSS } // link our main style sheet
        Link { rel: "stylesheet", href: TAILWIND_CSS } // link our TailwindCSS



        div {
            "data-theme": theme_data,
            class: "min-h-screen flex flex-col items-center justify-center gap-4 mt-5",

            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    let new_theme = format!(
                        "{}",
                        if theme_data() == "cupcake" { "aqua" } else { "cupcake" },
                    );
                    theme_data.set(new_theme.clone());
                    web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .document_element()
                        .unwrap()
                        .set_attribute("data-theme", &new_theme)
                        .unwrap();
                },
                "Theme ({theme_data})"
            
            }
            preview::Preview{}

        }
    }
}


