//! Module that has functions and structs that handles the Askama templates
//! to be rendered as HTML responses by Axum.
//!
use crate::data::{random_quote, Quote};
use axum_browser_adapter::wasm_compat;

/// Returns the rendered askama template for a random quote HTML element.
#[wasm_compat]
pub async fn quote() -> String {
    let Quote { text, author } = random_quote().await;
    format!(
        r#"
        <blockquote id="quote" class="text-center">
          <p class="text-2xl font-semibold md:text-4xl">{text}</p>
          <footer class="mt-4 text-xl text-right italic">{author}</footer>
        </blockquote>
        "#
    )
}

/// Returns a plain text random quote without any HTML.
#[wasm_compat]
pub async fn plain_quote() -> String {
    let Quote { text, author } = random_quote().await;
    let formatted_quote: String = format!("{text}\n\n - {author}\n");
    formatted_quote
}
