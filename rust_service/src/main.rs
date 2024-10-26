use warp::Filter;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin() 
        .allow_methods(vec!["GET"])
        .allow_headers(vec!["content-type"]);

    // Health Check Route
    let health_check = warp::path!("health")
        .map(|| warp::reply::json(&"OK"));

    // Hello World HTML Format
    let hello = warp::path!("hello" / "world")
        .map(|| warp::reply::html("<h1>Hello, World!</h1>"));

    // CORS for Route
    let routes = health_check.with(cors.clone()).or(hello.with(cors));

    // Starting the server 3030
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
