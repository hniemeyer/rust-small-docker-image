use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Setting up webserver.");
    // GET /
    let hello_world = warp::path::end().map(|| "Hello, World at root!");

    // GET /hello/warp => 200 OK with body "Hello, warp!"

    let hello = warp::path!("hello" / String).map(|name| format!("Hallo, {}!", name));

    let routes = warp::get().and(hello_world.or(hello));

    println!("Starting webserver at port 8000!");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
