//! Module that has functions and components that handles the rendering of
//! the quote and the button to refresh the quote.

use dioxus::prelude::*;

use crate::data::{random_quote, Quote};

#[cfg(debug_assertions)]
use log::info;

/// Quote
#[component]
pub fn Quote(cx: Scope) -> Element {
    let quote = use_shared_state::<Quote>(cx)
        .expect("failed to get quote shared state")
        .read();
    let text = &quote.text;
    let author = &quote.author;
    render! {
        blockquote {
            id: "quote",
            class: "text-center",
          p {
                class: "text-2xl font-semibold md:text-4xl",
                "{text}"
            }
          footer {
                class: "mt-4 text-xl text-right italic",
                "{author}"
            }
        }
    }
}

/// Button that triggers the quote refresh.
#[component]
pub fn Button(cx: Scope) -> Element {
    let quote = use_shared_state::<Quote>(cx).expect("failed to get quote shared state");

    render! {
        button {
            aria_label: "Refresh",
            class: "btn btn-accent bg-base-300 mt-10",
            onclick: move |_| {
                #[cfg(debug_assertions)]
                info!("Generated a new quote");

                *quote.write() = random_quote();
            },
            svg {
                class: "w-6 h-6 text-white",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99",
                }
            }
        }
    }
}
