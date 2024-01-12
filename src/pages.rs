//! Module that has functions and structs that handles the Askama templates
//! to be rendered as HTML responses by Axum.

use crate::data::{random_quote, Quote};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

/// A wrapper type that we'll use to encapsulate HTML parsed by askama
/// into valid HTML for axum to serve.
pub struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum
/// to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

/// An askama template that we'll use to render the root HTML element.
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

/// An askama template that we'll use to render the quote HTML elements.
/// It has htmx attributes that allow us to refresh the quote without
/// refreshing the entire page.
#[derive(Template)]
#[template(path = "quote.html")]
struct QuoteTemplate {
    text: String,
    author: String,
}

/// Returns the rendered askama template for the root HTML element.
pub async fn root() -> Response {
    let template = IndexTemplate {};
    HtmlTemplate(template).into_response()
}

/// Returns the rendered askama template for a random quote HTML element.
pub async fn quote() -> Response {
    let Quote { text, author } = random_quote();
    let template = QuoteTemplate { text, author };
    HtmlTemplate(template).into_response()
}

/// Returns a plain text random quote without any HTML.
pub async fn plain_quote() -> Response {
    let Quote { text, author } = random_quote();
    let formatted_quote: String = format!("\"{}\"\n - {}", text, author);
    Html(formatted_quote).into_response()
}
