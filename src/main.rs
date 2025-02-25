use warp::Filter;
use serde::Serialize;

#[derive(Serialize)]
struct HelloWorldResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| {
        let response = HelloWorldResponse {
            message: "Hello, World!".to_string(),
        };
        warp::reply::json(&response)
    });

    println!("Please open http://127.0.0.1:1984");
    warp::serve(hello).run(([127, 0, 0, 1], 1984)).await;
}