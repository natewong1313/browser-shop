use anyhow::anyhow;
use chromiumoxide::BrowserConfig;
use futures::StreamExt;
use port_check::free_local_port;
use std::sync::Arc;
use uuid::Uuid; // need this for handler polling

pub struct BrowserSession {
    id: Uuid,
    browser: chromiumoxide::Browser,
    handler: chromiumoxide::Handler,
}
impl BrowserSession {
    pub async fn launch() -> anyhow::Result<Self> {
        let id = Uuid::new_v4();

        let Some(free_port) = free_local_port() else {
            return Err(anyhow!("Could not get a free local port"));
        };
        tracing::debug!("Launching browser @ 127.0.0.1:{}", free_port);
        let port_arg = format!("--remote-debugging-port={}", free_port);

        let config = match BrowserConfig::builder().with_head().arg(port_arg).build() {
            Ok(config) => config,
            // it returns an error as a string -_-
            Err(err) => return Err(anyhow!("Unknown config error: {}", err)),
        };
        let (browser, handler) = chromiumoxide::Browser::launch(config).await?;

        Ok(Self {
            id,
            browser,
            handler: handler,
        })
    }

    // Poll the handler
    pub async fn run(&mut self) -> anyhow::Result<()> {
        while let Some(event) = self.handler.next().await {
            if let Err(err) = event {
                self.cleanup().await;
                return Err(anyhow!("unexpected browser handler error: {}", err));
            }
        }
        Ok(())
    }

    pub fn ws_addr(&mut self) -> &String {
        return self.browser.websocket_address();
    }

    pub async fn cleanup(&mut self) {
        if let Err(e) = self.browser.close().await {
            tracing::warn!("unexpected error closing browser: {}", e);
        }
    }
}
