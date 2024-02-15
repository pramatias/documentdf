use warp::{Filter, http::Response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyJson {
    a: String,
}

pub async fn serve() {
    // Define a filter for handling both GET and POST requests to /hello
    let hello = warp::path!("hello" / String)
        .and(warp::post())
        .and(warp::body::json())
        .map(|name: String, json: MyJson| {
            println!("Received POST message: {} with data1: {:?}", name, json);
            Response::builder()
                .header("content-type", "text/plain")
                .body(format!("Received POST message: {} with data: {:?}", name, json))
                .unwrap()
        })
        .or(warp::get().map(|| { let name = "asd";
            println!("Received GET message: {}", name);
            Response::builder()
                .header("content-type", "text/plain")
                .body(format!("Received GET message: {}", name))
                .unwrap()
        }));

    // Serve the combined routes
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
