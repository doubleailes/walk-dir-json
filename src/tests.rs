use super::rocket;
use quiet_stroll::InputPath;
use rocket::http::Status;
use rocket::local::blocking::Client;
use std::env;

#[test]
fn hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}
#[test]
fn test_walk() {
    let message = InputPath::new("./samples".to_string());
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/walk").json(&message).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let os = env::consts::OS;
    if os == "linux" {
        assert_eq!(response.into_string().unwrap().replace("\\\\", "/"), "{\"paths_list\":[\"./samples\",\"./samples/aaa.001.tif\",\"./samples/aaa.002.tif\",\"./samples/aaa.003.tif\",\"./samples/aaa.004.tif\",\"./samples/aaa.005.tif\",\"./samples/bbb.001.exr\",\"./samples/subfolder\",\"./samples/subfolder/ccc.050.exr\"]}");
    } else if os == "windows" {
        assert_eq!(response.into_string().unwrap().replace("\\\\", "/"), "{\"paths_list\":[\"./samples/aaa.001.tif\",\"./samples/aaa.002.tif\",\"./samples/aaa.003.tif\",\"./samples/aaa.004.tif\",\"./samples/aaa.005.tif\",\"./samples/bbb.001.exr\",\"./samples/subfolder\",\"./samples/subfolder/ccc.050.exr\",\"./samples\"]}");
    }
}
#[test]
fn test_listdir() {
    let message = InputPath::new("./samples/".to_string());
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/listdir").json(&message).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap().replace("\\\\", "/"),"{\"paths_list\":[\"./samples/aaa.001.tif\",\"./samples/aaa.002.tif\",\"./samples/aaa.003.tif\",\"./samples/aaa.004.tif\",\"./samples/aaa.005.tif\",\"./samples/bbb.001.exr\",\"./samples/subfolder\"]}");
}
#[test]
fn test_glob() {
    let message = InputPath::new("./samples/*.tif".to_string());
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/glob").json(&message).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap().replace("\\\\", "/"),"{\"paths_list\":[\"samples/aaa.001.tif\",\"samples/aaa.002.tif\",\"samples/aaa.003.tif\",\"samples/aaa.004.tif\",\"samples/aaa.005.tif\"]}");
}
