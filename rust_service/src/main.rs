use warp::Filter;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin() 
        .allow_methods(vec!["GET"])
        .allow_headers(vec!["content-type"]);

    // Define the route for the "Hello, World!" response
    let hello = warp::path!("hello" / "world")
        .map(|| warp::reply::html("<h1>Hello, World!</h1>"));


    warp::serve(hello.with(cors))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
