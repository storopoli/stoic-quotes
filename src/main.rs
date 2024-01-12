use anyhow::{Context, Result};
use axum::serve;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod app;
mod data;
mod pages;

use app::app;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up logging
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = app();

    // run our app with hyper, listening globally on port 80
    // FIXME: use https port 443
    let port = 80_u16;
    let addr = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    info!("router initialized, listening on port {:?}", port);
    serve(addr, app)
        .await
        .context("error while starting server")?;
    Ok(())
}
