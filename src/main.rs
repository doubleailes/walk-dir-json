//! # quiet-stroll
//!
//! ## Decription
//!
//! This repository is intend to create a POC of using rust to deliver client/server FS tools to:
//!
//! - **walk**, crawl the file system from an entrypoint in the file tree
//! - **listdir**, simply list the files in a directory
//! - **glob**, use glob
//!
#[macro_use]
extern crate rocket;
use quiet_stroll::{InputPath, QuietPaths};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
#[cfg(test)]
mod tests;

#[openapi(tag = "Default")]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[openapi(tag = "Fun")]
#[get("/coffee")]
/// # coffee
///
/// ## Description
///
/// Coffee is a fun function to use
///  [HTCPCP](https://en.wikipedia.org/wiki/Hyper_Text_Coffee_Pot_Control_Protocol)
/// and generate a error 418
fn coffee() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}
fn commun_manipulations(
    input_paths: QuietPaths,
    packed: Option<bool>,
    windows: Option<bool>,
) -> Json<QuietPaths> {
    let mut path_packed = if packed.unwrap_or(false) {
        input_paths.packed()
    } else {
        input_paths
    };
    let path_windows = if windows.unwrap_or(false) {
        path_packed.convert_windows();
        path_packed
    } else {
        path_packed
    };
    Json(path_windows)
}
#[openapi(tag = "FileSystem")]
#[post(
    "/walk?<packed>&<windows>",
    format = "application/json",
    data = "<input_path>"
)]
/// # walk
///
/// ## Description
///
/// Walk the directories from the entrypoint and return a Json of the paths
///
/// ## Tips
///
/// It is recommanded to use path with slash `/` instead of backslash `\`
///
/// ## Parameters
///
/// ### packed
///
/// You can use a filter `packed=true` or `packed=true` to pack frame sequences
/// 
///  ### windows
///
/// You can force to accept windows path by using windows filter
fn fwalk(
    input_path: Json<InputPath>,
    packed: Option<bool>,
    windows: Option<bool>,
) -> Json<QuietPaths> {
    commun_manipulations(QuietPaths::from_walk(input_path), packed, windows)
}
#[openapi(tag = "FileSystem")]
#[post(
    "/listdir?<packed>&<windows>",
    format = "application/json",
    data = "<input_path>"
)]
/// # listdir
///
/// ## Description
///
/// List the files and directory and return a Json of the paths
///
/// ## Tips
///
/// It is recommanded to use path with slash `/` instead of backslash `\`
///
/// ## Parameters
///
/// ### packed
///
/// You can use a filter `packed=true` or `packed=true` to pack frame sequences
/// 
///  ### windows
///
/// You can force to accept windows path by using windows filter
fn flistdir(
    input_path: Json<InputPath>,
    packed: Option<bool>,
    windows: Option<bool>,
) -> Json<QuietPaths> {
    commun_manipulations(QuietPaths::from_listdir(input_path), packed, windows)
}

#[openapi(tag = "FileSystem")]
#[post(
    "/glob?<packed>&<windows>",
    format = "application/json",
    data = "<input_path>"
)]
/// # glob
///
/// ## Description
///
/// Use a glob pattern to return a Json of the paths
///
/// ## Tips
///
/// It is recommanded to use path with slash `/` instead of backslash `\`
///
/// ## Parameters
///
/// ### packed
///
/// You can use a filter `packed=true` or `packed=true` to pack frame sequences
/// 
///  ### windows
///
/// You can force to accept windows path by using windows filter
///
/// ### Error
///
/// If you use wrongly a pattern. It will retur the error message from as a
/// paylod
fn fglob(
    input_path: Json<InputPath>,
    packed: Option<bool>,
    windows: Option<bool>,
) -> Result<Json<QuietPaths>, Custom<String>> {
    match QuietPaths::from_glob(input_path) {
        Ok(val) => Ok(commun_manipulations(val, packed, windows)),
        Err(err) => {
            // Construct a 400 Bad Request response with the error message
            let response = Custom(Status::BadRequest, format!("Error: {}", err));
            Err(response)
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            openapi_get_routes![index, flistdir, fglob, fwalk, coffee],
        )
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
