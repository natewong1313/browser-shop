use anyhow::Context;
use axum::{Router, routing::get};
use chromiumoxide::browser::{Browser, BrowserConfig};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use futures::StreamExt;

use crate::launcher::launch_browser;
pub mod launcher;
pub mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    server::serve().await.context("Server exploded").unwrap();

    // let mut browser_details = launch_browser().await?;
    // let mut browser = browser_details.browser;
    //
    // // spawn a new task that continuously polls the handler
    // let handle = tokio::spawn(async move {
    //     while let Some(h) = browser_details.handler.next().await {
    //         if h.is_err() {
    //             break;
    //         }
    //     }
    // });
    //
    // handle.await?;
    // Ok(())

    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    //
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:6700").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

