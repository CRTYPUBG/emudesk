//! EmuDesk coordination server entrypoint.
//!
//! Reads configuration from the environment (`PORT`, `BIND`, `API_TOKEN`,
//! `SERVER_DATA_DIR`) and runs the HTTP + WebSocket rendezvous server.

use emudesk_server::{run, ServerSettings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = ServerSettings::from_env();
    run(settings).await
}
