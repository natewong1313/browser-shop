use anyhow::anyhow;
use chromiumoxide::{Browser, BrowserConfig, Handler};
use futures::StreamExt; // need this for handler polling
use port_check::free_local_port;

// Handles launching a new browser on a free port
pub async fn launch_browser() -> anyhow::Result<(Browser, Handler)> {
    // TODO: check if this works properly in prod
    let Some(free_port) = free_local_port() else {
        return Err(anyhow!("Could not get a free local port"));
    };
    let port_arg = format!("--remote-debugging-port={}", free_port);
    tracing::debug!("Launching browser at {}", free_port);

    // Creates a new headed browser with a custom port
    // TODO: toggle headed / headless based on environment
    let config = match BrowserConfig::builder().with_head().arg(port_arg).build() {
        Ok(config) => config,
        // it returns an error as a string -_-
        Err(err) => return Err(anyhow!("Unknown config error: {}", err)),
    };

    // Browser controls the actual chromium browser and handler is for the websocket stuff
    // (mut browser, mut handler)
    let browser_details = Browser::launch(config).await?;

    Ok(browser_details)
}

// Needed so coxide works
pub async fn poll_browser_handler(mut browser_handler: Handler) -> anyhow::Result<()> {
    while let Some(event) = browser_handler.next().await {
        if let Err(err) = event {
            return Err(anyhow!("unexpected browser handler error: {}", err));
        }
    }
    Ok(())
}
