use crate::HtmlTemplate;
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

pub async fn root() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}
