//! Module that has functions and components that handles the rendering of
//! the whole app.

use dioxus::prelude::*;
use document::Stylesheet;

use crate::{
    component::{Button, Quote},
    data::random_quote,
};

/// Main App component that renders the whole app.
#[component]
pub fn App() -> Element {
    let quote = use_signal(random_quote);
    rsx! {
        Stylesheet { href: asset!("./assets/main.css") }
        div {
            class: "min-h-screen flex flex-col items-center justify-center text-white",
            div {
                class: "card card-bordered border-accent bg-base-300 shadow-2xl mx-4 p-10 rounded-lg",
                Quote { quote }
            }

            Button { quote }

            a {
                aria_label: "Go to the GitHub repository with the code",
                class: "mt-10 flex hover:text-accent",
                href: "https://github.com/storopoli/stoic-quotes",
                target: "_blank",
                rel: "noopener noreferrer",
                svg {
                    class: "w-8 h-8",
                    fill: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        fill: "evenodd",
                        clip: "evenodd",
                        d: "M12 2C6.477 2 2 6.477 2 12c0 4.418 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.009-.866-.014-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.462-1.11-1.462-.908-.62.069-.608.069-.608 1.003.07 1.532 1.03 1.532 1.03.891 1.529 2.341 1.088 2.912.833.091-.646.349-1.086.635-1.337-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.03-2.682-.103-.253-.447-1.27.098-2.646 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 7.07c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.026 2.747-1.026.547 1.376.203 2.394.1 2.646.64.699 1.028 1.591 1.028 2.682 0 3.841-2.337 4.687-4.565 4.934.359.31.678.92.678 1.852 0 1.336-.012 2.415-.012 2.741 0 .267.18.578.688.48A10.017 10.017 0 0022 12C22 6.477 17.523 2 12 2z",
                    }
                }
                "storopoli/stoic-quotes"
            }

        }
    }
}
