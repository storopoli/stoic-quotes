//! # Stoic Quotes
//!
//! `stoic-quotes` is a collection of stoic quotes in an Axum web server
//! that serves stoic quotes with reactivity by the all-mighty
//! [htmx](https://htmx.org) (no YAVASCRIPT).
//!
//! It also has plain-text API GET endpoints at `/` that returns a stoic quote
//! for terminal users with `curl` and `wget`.

pub mod data;
pub mod pages;

use crate::pages::{home, plain_quote, quote};
use prest::header::USER_AGENT;
use prest::*;

/// Handles the User Agent header
/// If the user agent is `curl` or `wget`,
/// return a plain quote.
/// Otherwise, return the root page.
async fn handle_user_agent<T>(req: Request<T>) -> Markup {
    let header = Request::headers(&req);
    let user_agent: String = if let Some(user_agent) = header.get(USER_AGENT) {
        user_agent.clone().to_str().unwrap().to_string()
    } else {
        "blank".to_string()
    };

    info!("got user agent: {user_agent}");

    if user_agent.contains("curl") || user_agent.contains("Wget") {
        html! {(plain_quote().await)}
    } else {
        home().await
    }
}

pub async fn into_page(content: Markup) -> Markup {
    let title = "Stoic Quotes";
    let css = "/assets/main.css";
    html! {(DOCTYPE) {
        (Head::default().title(title).css(css))
        (content)
        (Scripts::default())
    }}
}

/// Creates an Axum [`Router`] that only handles GET requests to
/// `/` and `/quote`.
pub fn app() -> Router {
    // Create a router
    info!("initializing router...");
    Router::new()
        .route("/", get(handle_user_agent))
        .route("/quote", get(quote))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    app().handle_fetch_events()
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt; // for `collect`
    use prest::{
        http::{Request, StatusCode},
        Body,
    };
    use std::str::from_utf8;
    use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn get_root() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Ensuring HTML is in the response by looking for typical HTML tags.
        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = from_utf8(&body_bytes).unwrap();
        assert!(
            body_str.contains('<') && body_str.contains('>'),
            "Body doesn't contain HTML: {body_str}",
        );
    }

    #[tokio::test]
    async fn get_quote() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/quote")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Ensuring HTML is in the response by looking for typical HTML tags.
        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = from_utf8(&body_bytes).unwrap();
        assert!(
            body_str.contains('<') && body_str.contains('>'),
            "Body doesn't contain HTML: {body_str}",
        );
    }

    #[tokio::test]
    async fn get_unknown() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/foo").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn post_root() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn post_quote() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/quote")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED)
    }

    #[tokio::test]
    async fn post_unknown() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/foo")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn multiple_request() {
        let mut app = app().into_service();

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_root_curl() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .header("User-Agent", "curl/8.4.0")
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Ensuring no HTML is in the response by looking for typical HTML tags.
        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = from_utf8(&body_bytes).unwrap();
        assert!(
            !body_str.contains('<') && !body_str.contains('>'),
            "Body contains HTML: {body_str}",
        );
    }

    #[tokio::test]
    async fn get_root_wget() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .header("User-Agent", "Wget/1.21.4")
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Ensuring no HTML is in the response by looking for typical HTML tags.
        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = from_utf8(&body_bytes).unwrap();
        assert!(
            !body_str.contains('<') && !body_str.contains('>'),
            "Body contains HTML: {body_str}",
        );
    }

    #[tokio::test]
    async fn get_quote_htmx() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/quote")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Ensuring that we have id = "quote" so that htmx can do its thing
        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = from_utf8(&body_bytes).unwrap();
        assert!(
            body_str.contains(r#"id="quote""#),
            r#"Body does not contain id="quote": {body_str}"#,
        );
    }
}
