use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the route for the "Hello, World!" response
    let hello = warp::path!("hello" / "world")
        .map(|| warp::reply::html("<h1>Hello, World!</h1>"));

    // Start the server
    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
