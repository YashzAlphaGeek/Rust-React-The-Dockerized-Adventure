use warp::Filter;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin() 
        .allow_methods(vec!["GET"])
        .allow_headers(vec!["content-type"]);

    // Define the health check route
    let health_check = warp::path!("health")
        .map(|| warp::reply::json(&"OK"));

    // Define the route for the "Hello, World!" response
    let hello = warp::path!("hello" / "world")
        .map(|| warp::reply::html("<h1>Hello, World!</h1>"));

    // Combine routes and apply CORS, cloning `cors` for each route
    let routes = health_check.with(cors.clone()).or(hello.with(cors));

    // Start the server on port 3030
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
