//! Module that has functions that handles the Axum [`Router`].

use crate::pages::{plain_quote, quote, root};
use axum::{http::header::USER_AGENT, http::Request, response::IntoResponse, routing::get, Router};
use std::{env::current_dir, path::PathBuf};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

/// Handles the User Agent header
/// If the user agent is `curl` or `wget`,
/// return a plain quote.
/// Otherwise, return the root page.
async fn handle_user_agent<T>(req: Request<T>) -> impl IntoResponse {
    let header = Request::headers(&req);
    let user_agent: String = if let Some(user_agent) = header.get(USER_AGENT) {
        user_agent.clone().to_str().unwrap().to_string()
    } else {
        "blank".to_string()
    };

    info!("got user agent: {user_agent}");

    if user_agent.contains("curl") || user_agent.contains("Wget") {
        plain_quote().await.into_response()
    } else {
        root().await.into_response()
    }
}

/// Creates an Axum [`Router`] that only handles GET requests to
/// `/` and `/quote`.
pub fn app() -> Router {
    let assets_path: PathBuf = current_dir().unwrap();
    // Create a router
    info!("initializing router...");
    Router::new()
        .route("/", get(handle_user_agent))
        .route("/quote", get(quote))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        // We can still add middleware
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
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
