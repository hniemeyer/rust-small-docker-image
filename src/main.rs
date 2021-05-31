use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    println!("Setting up webserver.");
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hallo, {}!", name));

    println!("Starting webserver at port 8000!");
    warp::serve(hello)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
