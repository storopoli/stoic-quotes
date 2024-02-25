//! Module that has functions and structs that handles the Askama templates
//! to be rendered as HTML responses by Axum.

use crate::{
    data::{random_quote, Quote},
    into_page,
};
use prest::{html, Markup};

pub async fn quote() -> Markup {
    let quote = random_quote();
    let text = &quote.text;
    let author = &quote.author;

    html! {
        blockquote #quote .text-center {
            p ."text-2xl font-semibold md:text-4xl" {
                 (text)
            }
            footer ."mt-4 text-xl text-right italic" {
                (author)
            }
        }
    }
}

/// Returns the rendered askama template for the root HTML element.
pub async fn home() -> Markup {
    into_page(html!(
        div."min-h-screen flex flex-col items-center justify-center text-white" {
            div."card card-bordered border-accent bg-base-300 shadow-2xl mx-4 p-10 rounded-lg" {
                blockquote hx-get="/quote" hx-swap="outerHTML" hx-trigger="load" #quote ."text-center" {
                    "Please Enable JavaScript (I know, it sucks...)"
                }
            }
            button."btn btn-accent bg-base-300 mt-10"  hx-get="/quote" hx-trigger="click" hx-target="#quote" hx-swap="outerHTML transition:true"
                aria-label="Refresh" {
                svg."w-6 h-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                    stroke-width="1.5" stroke="currentColor" {
                    path stroke-linecap="round" stroke-linejoin="round"
                        d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99" {}
                }
            }
            a."mt-10 flex hover:text-accent" aria-label="Go to the GitHub repository with the code"
                href="https://github.com/storopoli/stoic-quotes" target="_blank" rel="noopener noreferrer" {
                svg ."w-8 h-8" fill="currentColor" viewBox="0 0 24 24" {
                    path fill="evenodd" clip="evenodd"
                        d="M12 2C6.477 2 2 6.477 2 12c0 4.418 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.009-.866-.014-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.462-1.11-1.462-.908-.62.069-.608.069-.608 1.003.07 1.532 1.03 1.532 1.03.891 1.529 2.341 1.088 2.912.833.091-.646.349-1.086.635-1.337-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.03-2.682-.103-.253-.447-1.27.098-2.646 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 7.07c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.026 2.747-1.026.547 1.376.203 2.394.1 2.646.64.699 1.028 1.591 1.028 2.682 0 3.841-2.337 4.687-4.565 4.934.359.31.678.92.678 1.852 0 1.336-.012 2.415-.012 2.741 0 .267.18.578.688.48A10.017 10.017 0 0022 12C22 6.477 17.523 2 12 2z" {}
                }
                "storopoli/stoic-quotes"
            }
        }
    )).await
}

/// Returns a plain text random quote without any HTML.
pub async fn plain_quote() -> String {
    let Quote { text, author } = random_quote();
    let formatted_quote: String = format!("{text}\n\n - {author}\n");
    formatted_quote
}
