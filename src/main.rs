use dropshot::endpoint;
use dropshot::ApiDescription;
use dropshot::ConfigDropshot;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::HttpError;
use dropshot::HttpResponseOk;
use dropshot::HttpResponseUpdatedNoContent;
use dropshot::HttpServerStarter;
use dropshot::RequestContext;
use dropshot::TypedBody;
use glob::glob;
use jwalk::WalkDir;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;

#[derive(Deserialize, Serialize, JsonSchema)]
struct Paths {
    paths_list: Vec<String>,
}
fn get_paths(input_path: &str) -> Vec<String> {
    let items: Vec<String> = WalkDir::new(input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect();
    items
}

fn get_list_dir(input_path: &str) -> Vec<String> {
    let items: Vec<String> = fs::read_dir(input_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect();
    items
}
fn get_glob(input_path: &str) -> Vec<String> {
    let items: Vec<String> = glob(input_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|x| x.display().to_string())
        .collect();
    items
}

#[tokio::main]
async fn main() -> Result<(), String> {
    /*
     * We must specify a configuration with a bind address.  We'll use 127.0.0.1
     * since it's available and won't expose this server outside the host.  We
     * request port 0, which allows the operating system to pick any available
     * port.
     */
    let port = match env::var_os("PORT") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$PORT is not set"),
    };
    let ip_address = match env::var_os("IP_ADDRESS") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$IP_ADDRESS is not set"),
    };
    let config_dropshot: ConfigDropshot = ConfigDropshot {
        bind_address: format!("{}:{}", ip_address, port).parse().unwrap(),
        request_body_max_bytes: 1024,
        tls: None,
    };

    /*
     * For simplicity, we'll configure an "info"-level logger that writes to
     * stderr assuming that it's a terminal.
     */
    let config_logging = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    };
    let log = config_logging
        .to_logger("example-basic")
        .map_err(|error| format!("failed to create logger: {}", error))?;

    /*
     * Build a description of the API.
     */
    let mut api = ApiDescription::new();
    api.register(fwalk).unwrap();
    api.register(flistdir).unwrap();
    api.register(fglob).unwrap();

    /*
     * The functions that implement our API endpoints will share this context.
     */
    let api_context = ExampleContext::new();

    /*
     * Set up the server.
     */
    let server = HttpServerStarter::new(&config_dropshot, api, api_context, &log)
        .map_err(|error| format!("failed to create server: {}", error))?
        .start();

    /*
     * Wait for the server to stop.  Note that there's not any code to shut down
     * this server, so we should never get past this point.
     */
    server.await
}

/**
 * Application-specific example context (state shared by handler functions)
 */
struct ExampleContext {
    paths_list: Vec<String>,
}

impl ExampleContext {
    /**
     * Return a new ExampleContext.
     */
    pub fn new() -> ExampleContext {
        ExampleContext {
            paths_list: Vec::<String>::new(),
        }
    }
}

/*
 * HTTP API interface
 */

#[derive(Deserialize, Serialize, JsonSchema)]
struct Path {
    input_path: String,
}
/**
 * Fetch the walk path.
 */
#[endpoint {
    method = PUT,
    path = "/walk",
}]
async fn fwalk(
    rqctx: Arc<RequestContext<ExampleContext>>,
    update: TypedBody<Path>,
) -> Result<HttpResponseOk<Paths>, HttpError> {
    let _api_context = rqctx.context();
    let updated_value = update.into_inner();

    Ok(HttpResponseOk(Paths {
        paths_list: get_paths(&updated_value.input_path),
    }))
}

#[endpoint {
    method = PUT,
    path = "/listdir",
}]
async fn flistdir(
    rqctx: Arc<RequestContext<ExampleContext>>,
    update: TypedBody<Path>,
) -> Result<HttpResponseOk<Paths>, HttpError> {
    let _api_context = rqctx.context();
    let updated_value = update.into_inner();

    Ok(HttpResponseOk(Paths {
        paths_list: get_list_dir(&updated_value.input_path),
    }))
}

#[endpoint {
    method = PUT,
    path = "/glob",
}]
async fn fglob(
    rqctx: Arc<RequestContext<ExampleContext>>,
    update: TypedBody<Path>,
) -> Result<HttpResponseOk<Paths>, HttpError> {
    let _api_context = rqctx.context();
    let updated_value = update.into_inner();

    Ok(HttpResponseOk(Paths {
        paths_list: get_glob(&updated_value.input_path),
    }))
}
