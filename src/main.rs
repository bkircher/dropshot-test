use dropshot::ApiDescription;
use dropshot::ConfigDropshot;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::HttpServerStarter;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), String> {
    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("minimal-example")
    .map_err(|e| e.to_string())?;

    let api = ApiDescription::new();

    let server = HttpServerStarter::new(
        &ConfigDropshot {
            bind_address: "127.0.0.1:0".parse().unwrap(),
            request_body_max_bytes: 1024,
        },
        api,
        Arc::new(()),
        &log,
    )
    .map_err(|error| format!("failed to start server: {}", error))?
    .start();
    server.await
}
