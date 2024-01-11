use crate::data::{random_quote, Quote};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

/// A wrapper type that we'll use to encapsulate HTML parsed by askama
/// into valid HTML for axum to serve.
pub struct HtmlTemplate<T>(T);

// Allows us to convert Askama HTML templates into valid HTML for axum
// to serve in the response.
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

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "quote.html")]
struct QuoteTemplate {
    text: String,
    author: String,
}

pub async fn root() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

pub async fn quote() -> impl IntoResponse {
    let Quote { text, author } = random_quote();
    let template = QuoteTemplate { text, author };
    HtmlTemplate(template)
}
