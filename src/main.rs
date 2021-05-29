use dropshot::endpoint;
use dropshot::ApiDescription;
use dropshot::ConfigDropshot;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::HttpError;
use dropshot::HttpResponseOk;
use dropshot::HttpServerStarter;
use dropshot::RequestContext;
use schemars::JsonSchema;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize, JsonSchema)]
struct Guest {
    /** name of this guest */
    name: String,
}

#[endpoint {
    method = GET,
    path = "guests/guest1"
}]
async fn get_guest(_rqctx: Arc<RequestContext<()>>) -> Result<HttpResponseOk<Guest>, HttpError> {
    let guest = Guest {
        name: String::from("some guest"),
    };
    Ok(HttpResponseOk(guest))
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("minimal-example")
    .map_err(|e| e.to_string())?;

    let mut api = ApiDescription::new();
    api.register(get_guest).unwrap();

    let server = HttpServerStarter::new(
        &ConfigDropshot {
            bind_address: "127.0.0.1:8080".parse().unwrap(),
            request_body_max_bytes: 1024,
        },
        api,
        *Arc::new(()),
        &log,
    )
    .map_err(|error| format!("failed to start server: {}", error))?
    .start();
    server.await
}
