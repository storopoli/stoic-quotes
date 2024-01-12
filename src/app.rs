use crate::pages::{plain_quote, quote, root};
use anyhow::{Context, Result};
use axum::{http::header::USER_AGENT, http::Request, response::Response, routing::get, Router};
use tracing::info;

/// Handles the User Agent header
/// If the user agent is `curl` or `wget`,
/// return a plain quote.
/// Otherwise, return the root page.
async fn handle_user_agent<T>(req: Request<T>) -> Response {
    let header = Request::headers(&req);
    let user_agent: String = if let Some(user_agent) = header.get(USER_AGENT) {
        user_agent.clone().to_str().unwrap().to_string()
    } else {
        "blank".to_string()
    };

    info!("got user agent: {user_agent}");

    if user_agent.contains("curl") || user_agent.contains("Wget") {
        plain_quote().await
    } else {
        root().await
    }
}

pub async fn app() -> Result<()> {
    // Create a router
    info!("initializing router...");
    let app = Router::new()
        .route("/", get(handle_user_agent))
        .route("/quote", get(quote));

    // run our app with hyper, listening globally on port 80
    // FIXME: use https port 443
    let port = 80_u16;
    let addr = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    info!("router initialized, listening on port {:?}", port);
    axum::serve(addr, app)
        .await
        .context("error while starting server")?;
    Ok(())
}
