use anyhow::Context;
use axum::{http::header::USER_AGENT, http::Request, response::Response, routing::get, Router};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod data;
mod pages;
use pages::{plain_quote, quote, root};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up logging
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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
