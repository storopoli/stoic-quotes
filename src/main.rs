use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod pages;
use pages::root;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama
/// into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

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
    let app = Router::new().route("/", get(root));

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
